use crate::{
    ENGINE_CFG, MayFail,
    generators::parser::{DamageRange, ZERO, items::Item, runes::Rune},
    scripts::{
        batch::FormulaDoc,
        simplify::{Precision, approx_constant, format_number, simplify, simplify_exact},
    },
};
use databake::CrateEnv;
use heck::{ToPascalCase, ToSnakeCase};
use rand::{RngExt, SeedableRng, rngs::SmallRng};
use regex::Regex;
use serde::Serialize;
use std::{
    collections::{BTreeSet, HashSet},
    fmt::{Debug, Write},
    ops::{Index, Range},
    sync::LazyLock,
};
use tutorlolv2_types::{AttackType, Attrs, CtxVar, DamageIndex, DamageType, Tag};

pub const ZERO_FN: &str = "crate::libgen::zero";

pub trait BakeExt {
    /// Emit a Rust expression without relying on imports in the generated module.
    fn bake(&self) -> String;
}

impl<T: databake::Bake + ?Sized> BakeExt for T {
    fn bake(&self) -> String {
        databake::Bake::bake(self, &CrateEnv::default()).to_string()
    }
}

pub fn is_zero(value: &str) -> bool {
    value == ZERO || value == "0" || value == "0.0" || value == "(0.0)" || value == "(0)"
}

pub trait ItemOrRune {
    const TAG: Tag;

    fn id(&self) -> &str;
    fn damage_type(&self) -> DamageType;
    fn attributes(&self) -> Attrs;
}

impl ItemOrRune for Item {
    const TAG: Tag = Tag::Items;

    fn id(&self) -> &str {
        &self.data.item_id
    }

    fn damage_type(&self) -> DamageType {
        self.damage_type
    }

    fn attributes(&self) -> Attrs {
        self.attributes
    }
}

impl ItemOrRune for Rune {
    const TAG: Tag = Tag::Runes;

    fn id(&self) -> &str {
        &self.data.rune_id
    }

    fn damage_type(&self) -> DamageType {
        self.damage_type
    }

    fn attributes(&self) -> Attrs {
        Attrs::Undefined
    }
}

impl ItemOrRuneExt for Item {}
impl ItemOrRuneExt for Rune {}

pub struct ItemFormulaPlan {
    functions: [String; 4],
    aliases: [Option<(usize, String)>; 4],
    formulas: [String; 4],
    active: [bool; 4],
}

pub trait ItemOrRuneExt:
    Index<AttackType, Output = DamageRange> + ItemOrRune + Debug + Serialize
{
    fn identifiers(&self) -> Vec<CtxVar> {
        let mut set = BTreeSet::new();

        let mut add_set = |attack_type: AttackType, damage_index| {
            for element in get_identifiers(&self[attack_type][damage_index], self.damage_type()) {
                set.insert(element);
            }
        };

        add_set(AttackType::Melee, DamageIndex::Min);
        add_set(AttackType::Melee, DamageIndex::Max);
        add_set(AttackType::Ranged, DamageIndex::Min);
        add_set(AttackType::Ranged, DamageIndex::Max);

        set.into_iter().collect()
    }

    fn deals_damage(&self) -> Vec<bool> {
        [
            self[AttackType::Melee].deals_damage(),
            self[AttackType::Ranged].deals_damage(),
        ]
        .concat()
    }

    fn repr_metadata(&self) -> String {
        format!(
            "tutorlolv2_types::TypeMetadata {{
                kind: crate::libgen::{enum_name}::{id},
                damage_type: {damage_type},
                attributes: {attributes},
            }}",
            enum_name = Self::TAG.enum_name(),
            id = self.id(),
            damage_type = self.damage_type().bake(),
            attributes = self.attributes().bake(),
        )
    }

    fn function_names(&self) -> String {
        let functions = self.functions();
        let melee_fns = functions[0..2].join(",");
        let ranged_fns = functions[2..4].join(",");

        format!(
            "{ENGINE_CFG} melee: [{melee_fns}],
             {ENGINE_CFG} ranged: [{ranged_fns}],"
        )
    }

    fn eval(&self) -> String {
        let id = self.id();
        let module = id.to_snake_case();

        let deals_damage = [
            self[AttackType::Melee].deals_damage(),
            self[AttackType::Ranged].deals_damage(),
        ]
        .concat();

        let functions = self.functions();

        let get_arms = |range: Range<_>| {
            deals_damage[range.clone()]
                .iter()
                .zip(&functions[range])
                .map(|(v, function)| {
                    let f = match *v {
                        true => &format!(
                            "crate::libgen::{}::{module}::{}",
                            Self::TAG.as_ref(),
                            function
                        ),
                        false => ZERO_FN,
                    };

                    format!("{f}(ctx)")
                })
                .collect::<Vec<_>>()
                .join(",")
        };

        format!(
            "{enum_name}::{id} => {{
                match attack_type {{
                    {melee} => [{melee_arms}],
                    {ranged} => [{ranged_arms}]
                }}
            }},",
            melee = AttackType::Melee.bake(),
            ranged = AttackType::Ranged.bake(),
            enum_name = Self::TAG.enum_name(),
            melee_arms = get_arms(0..2),
            ranged_arms = get_arms(2..4),
        )
    }

    fn prepare_formulas(&self) -> ItemFormulaPlan {
        let melee = &self[AttackType::Melee];
        let ranged = &self[AttackType::Ranged];
        let raw = [
            &melee.min_dmg,
            &melee.max_dmg,
            &ranged.min_dmg,
            &ranged.max_dmg,
        ];

        let formulas = core::array::from_fn(|i| simplify(raw[i]));

        ItemFormulaPlan {
            functions: self.functions(),
            aliases: self.build_aliases(&formulas),
            formulas,
            active: core::array::from_fn(|i| !is_zero(raw[i])),
        }
    }

    fn build_aliases(&self, raw: &[String; 4]) -> [Option<(usize, String)>; 4] {
        let all_vars = self.identifiers();

        let mut aliases = [None, None, None, None];

        for i in 0..4 {
            if is_zero(&raw[i]) {
                continue;
            }

            for j in 0..i {
                if is_zero(&raw[j]) {
                    continue;
                }

                let ratio_sym = simplify(&format!("({}) / ({})", raw[i], raw[j]));

                let ratio = if !ratio_sym.contains("ctx") {
                    ratio_sym
                } else {
                    let Some(k) = probe_ratio(&raw[j], &raw[i], &all_vars) else {
                        continue;
                    };

                    k
                };

                aliases[i] = Some((j, ratio));
                break;
            }
        }

        aliases
    }

    fn closures(&self, plan: &ItemFormulaPlan) -> MayFail<String> {
        let functions = &plan.functions;
        let aliases = &plan.aliases;

        let mut seen = HashSet::new();
        let mut rust = String::new();

        for i in 0..4 {
            let body = &plan.formulas[i as usize];
            let f = &functions[i as usize];

            let default = !plan.active[i as usize];

            let formula_f32 = if default {
                continue;
            } else if let Some((base_idx, ref ratio)) = aliases[i as usize] {
                let base_fn = &functions[base_idx];

                cast_f32(&format!("{ratio} * {base_fn}(ctx)"))?
            } else {
                cast_f32(body)?
            };

            if !seen.contains(f) {
                seen.insert(f);

                let param = ctx_param(&formula_f32);

                write!(
                    rust,
                    "{ENGINE_CFG} pub const fn {f}({param}: &tutorlolv2_types::Ctx) -> f32 {{{formula_f32}}}"
                )?;
            }
        }

        Ok(rust)
    }

    fn formula_docs(&self, plan: &ItemFormulaPlan) -> Vec<FormulaDoc> {
        let id = self.id().to_snake_case();

        plan.formulas
            .iter()
            .enumerate()
            .filter_map(|(i, formula)| {
                if !plan.active[i] {
                    return None;
                }

                let source = if let Some((j, ratio)) = &plan.aliases[i] {
                    let function = format!("{id}_f{j}(ctx)");

                    if ratio == "1" {
                        function
                    } else {
                        format!("{ratio} * {function}")
                    }
                } else {
                    formula.clone()
                };

                Some(FormulaDoc {
                    local: i as u8,
                    function: format!("{id}_f{i}"),
                    source,
                    name: self.id().into(),
                    comment: String::new(),
                })
            })
            .collect()
    }

    fn functions(&self) -> [String; 4] {
        let value_id = self.id();
        let melee = &self[AttackType::Melee];
        let ranged = &self[AttackType::Ranged];

        let id = value_id.to_snake_case();

        let min = DamageIndex::Min;
        let max = DamageIndex::Max;

        let has_max = !is_zero(&melee[max]) || !is_zero(&ranged[max]);

        let min_shared = !is_zero(&melee[min]) && melee[min] == ranged[min];
        let max_shared = !is_zero(&melee[max]) && melee[max] == ranged[max];
        let min_suffix = if has_max { "_min" } else { "" };

        [
            if is_zero(&melee[min]) {
                ZERO_FN.into()
            } else if min_shared {
                format!("{id}{min_suffix}")
            } else {
                format!("{id}_melee{min_suffix}")
            },
            if is_zero(&melee[max]) {
                ZERO_FN.into()
            } else if max_shared {
                format!("{id}_max")
            } else {
                format!("{id}_melee_max")
            },
            if is_zero(&ranged[min]) {
                ZERO_FN.into()
            } else if min_shared {
                format!("{id}{min_suffix}")
            } else {
                format!("{id}_ranged{min_suffix}")
            },
            if is_zero(&ranged[max]) {
                ZERO_FN.into()
            } else if max_shared {
                format!("{id}_max")
            } else {
                format!("{id}_ranged_max")
            },
        ]
    }
}

pub fn get_identifiers(damage: &str, damage_type: DamageType) -> impl Iterator<Item = CtxVar> + '_ {
    static RE_IDENTS: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"ctx\.([a-z_][a-z0-9_]*)").unwrap());

    RE_IDENTS
        .captures_iter(damage)
        .filter_map(|cap| cap[1].to_pascal_case().parse().ok())
        .chain(match damage_type {
            DamageType::Physical => Some(CtxVar::PhysicalMultiplier),
            DamageType::Magic => Some(CtxVar::MagicMultiplier),
            _ => None,
        })
}

pub fn cast_f32(s: &str) -> MayFail<String> {
    use quote::ToTokens;
    use syn::{Expr, Lit, Member, visit_mut::VisitMut};

    struct FloatExpressions;

    impl VisitMut for FloatExpressions {
        // Patterns, including range bounds, must keep their integer literals.
        fn visit_pat_mut(&mut self, _: &mut syn::Pat) {}

        fn visit_expr_mut(&mut self, expr: &mut Expr) {
            syn::visit_mut::visit_expr_mut(self, expr);

            // Literals such as `3.14` are denied by `clippy::approx_constant`.
            if let Expr::Lit(syn::ExprLit {
                lit: Lit::Float(n), ..
            }) = expr
                && let Some(name) = approx_constant(n.base10_digits())
            {
                let name = quote::format_ident!("{name}");
                *expr = syn::parse_quote!(core::f32::consts::#name);
            }
        }

        fn visit_expr_lit_mut(&mut self, expr: &mut syn::ExprLit) {
            let digits = match &expr.lit {
                Lit::Int(n) if n.suffix().is_empty() => n.base10_digits(),
                Lit::Float(n) if n.suffix().is_empty() => n.base10_digits(),
                _ => return,
            };
            
            let Ok(value) = digits.parse::<f64>() else {
                return;
            };
            
            let literal = format_number(value, Precision::F32);
            expr.lit = Lit::Float(syn::LitFloat::new(&format!("{literal}f32"), expr.lit.span()));
        }

        fn visit_expr_match_mut(&mut self, expr: &mut syn::ExprMatch) {
            if let Expr::Field(field) = expr.expr.as_ref()
                && let Expr::Path(base) = field.base.as_ref()
                && base.path.is_ident("ctx")
                && let Member::Named(member) = &field.member
                && matches!(
                    member.to_string().as_str(),
                    "level" | "q_level" | "w_level" | "e_level" | "r_level"
                )
            {
                let discriminant = &expr.expr;
                expr.expr = syn::parse_quote!(#discriminant as u8);
            }
            syn::visit_mut::visit_expr_match_mut(self, expr);
        }
    }

    let mut expr = syn::parse_str::<Expr>(s).map_err(|e| format!("Invalid formula {s:?}: {e}"))?;

    FloatExpressions.visit_expr_mut(&mut expr);

    Ok(expr.into_token_stream().to_string())
}

pub fn ctx_param(s: &str) -> &'static str {
    if s.contains("ctx") { "ctx" } else { "_" }
}

pub fn probe_ratio(min_expr: &str, max_expr: &str, all_vars: &[CtxVar]) -> Option<String> {
    const N_PROBES: usize = 20;
    const MIN_VALID: usize = 8;
    const TOL: f64 = 1e-9;

    let mut rng = SmallRng::seed_from_u64(0x4c4f_4c42_5544_494c);
    let mut k_candidate: Option<f64> = None;
    let mut valid = 0usize;

    for _ in 0..N_PROBES {
        let mut min_s = min_expr.to_owned();
        let mut max_s = max_expr.to_owned();

        for var in all_vars {
            let v = rng.random_range(2..=998).to_string();
            min_s = min_s.replace(var.as_var(), &v);
            max_s = max_s.replace(var.as_var(), &v);
        }

        let Ok(min_v) = simplify_exact(&min_s).trim().parse::<f64>() else {
            continue;
        };

        let Ok(max_v) = simplify_exact(&max_s).trim().parse::<f64>() else {
            continue;
        };

        if min_v.abs() < 1e-10 {
            continue;
        }

        let ratio = max_v / min_v;
        valid += 1;

        match k_candidate {
            None => k_candidate = Some(ratio),
            Some(k) => {
                if (ratio - k).abs() / (1.0 + k.abs()) > TOL {
                    return None;
                }
            }
        }
    }

    if valid >= MIN_VALID {
        Some(k_candidate)
    } else {
        None
    }
    .flatten()
    .map(|k| format_number(k, Precision::F32))
}
