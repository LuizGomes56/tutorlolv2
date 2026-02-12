use crate::{
    CwdPath, Generated, GeneratorFn, Tracker, ZERO_FN_OFFSET, push_end,
    scripts::{StringExt, model::Rune, rustfmt_batch},
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::collections::{BTreeMap, HashMap};

struct RuneResult {
    name: String,
    name_pascal: String,
    name_ssnake: String,
    base_declaration: String,
    html_declaration: String,
    riot_id: usize,
    match_arm: String,
    idents: String,
    html_closure: String,
    deals_damage: bool,
}

const MAX_TUPLE: (usize, usize) = (usize::MAX, usize::MAX);

pub fn generate_runes() -> GeneratorFn {
    let data = {
        let json = CwdPath::deserialize::<HashMap<usize, Rune>>("internal/runes.json")?;

        let mut runes = json
            .into_par_iter()
            .map(|(riot_id, rune)| {
                let Rune {
                    name,
                    damage_type,
                    ref ranged,
                    ref melee,
                } = rune;

                let name_pascal = name.pascal_case();
                let name_ssnake = name.to_ssnake();

                let metadata = format!(
                    "TypeMetadata {{
                        kind: RuneId::{name_pascal},
                        damage_type: DamageType::{damage_type},
                        attributes: Attrs::Undefined
                    }}"
                );

                let normalize = |expr: &str| expr.clean().to_lowercase().cast_f32();

                let melee_body = normalize(melee);
                let ranged_body = normalize(ranged);

                let mut constfn_declaration = String::new();
                let single_damage = melee_body == ranged_body;

                let (melee_fn, ranged_fn) = match single_damage {
                    true => {
                        let fn_name = name_ssnake.to_lowercase();
                        constfn_declaration.push_str(&format!(
                            "pub const fn {fn_name}(ctx: &Ctx) -> f32 {{
                                {melee_body}
                            }}"
                        ));

                        (fn_name.clone(), fn_name)
                    }
                    false => {
                        let melee_fn = format!("{name_ssnake}_melee").to_lowercase();
                        let ranged_fn = format!("{name_ssnake}_ranged").to_lowercase();

                        constfn_declaration.push_str(&format!(
                            "pub const fn {melee_fn}(ctx: &Ctx) -> f32 {{
                                {melee_body}
                            }}"
                        ));

                        constfn_declaration.push_str(&format!(
                            "pub const fn {ranged_fn}(ctx: &Ctx) -> f32 {{
                                {ranged_body}
                            }}"
                        ));

                        (melee_fn, ranged_fn)
                    }
                };

                let mk_closure = |expr: &str| {
                    let arg = expr.ctx_param();
                    format!("|{arg}| {}", expr.to_lowercase())
                };

                let melee_closure = mk_closure(melee);
                let ranged_closure = mk_closure(ranged);

                let base_declaration = format!(
                    "pub static {name_ssnake}: CachedRune = CachedRune {{
                        name: {name:?},
                        damage_type: DamageType::{damage_type},
                        riot_id: {riot_id},
                        internal_id: RuneId::{name_pascal},
                        undeclared: false,
                        metadata: {metadata},"
                );

                let html_declaration = [
                    base_declaration.as_str(),
                    &match single_damage {
                        true => format!("damage: {melee_closure}}};"),
                        false => {
                            format!(
                                "melee_damage: {melee_closure}, ranged_damage: {ranged_closure} }};"
                            )
                        }
                    },
                ]
                .concat();

                let base_declaration = format!(
                    "{base_declaration}
                    melee_damage: {melee_fn},
                    ranged_damage: {ranged_fn} }};
                    {constfn_declaration}"
                );

                let match_arm = match single_damage {
                    true => format!("{melee_fn}(ctx)"),
                    false => format!(
                        "match attack_type {{
                            AttackType::Melee => {melee_fn}(ctx),
                            AttackType::Ranged => {ranged_fn}(ctx)
                        }}"
                    ),
                };

                let idents = (melee_closure.clone() + &ranged_closure)
                    .get_idents(&damage_type)
                    .into_iter()
                    .collect::<String>();

                println!("[build] RuneId::{name_pascal}");

                RuneResult {
                    riot_id,
                    match_arm,
                    html_declaration,
                    base_declaration,
                    name,
                    name_ssnake,
                    name_pascal,
                    html_closure: constfn_declaration
                        .replace("pub const ", "")
                        .trim()
                        .to_string(),
                    deals_damage: true,
                    idents: format!("&[{idents}]"),
                }
            })
            .collect::<Vec<_>>();

        let mut undeclared =
            CwdPath::deserialize::<BTreeMap<String, usize>>("internal/rune_names.json")?;

        for (name, riot_id) in [
            ("Ability Haste", 9994),
            ("Adaptive Force", 9990),
            ("Attack Speed", 9992),
            ("Eyeball Collection", 8120),
            ("Ghost Poro", 8136),
            ("Health", 9993),
            ("Health Scaling", 9991),
            ("Move Speed", 9996),
            ("Tenacity and Slow Resist", 9995),
            ("Zombie Ward", 8138),
        ] {
            undeclared.insert(name.to_string(), riot_id);
        }

        for (name, riot_id) in undeclared {
            let name_pascal = name.pascal_case();

            let mut repeated = false;
            for value in &runes {
                if value.name_pascal.to_lowercase() == name_pascal.to_lowercase() {
                    repeated = true;
                }
            }

            if repeated {
                continue;
            }

            let name_ssnake = name.to_ssnake();

            let metadata = format!(
                "TypeMetadata {{
                    kind: RuneId::{name_pascal},
                    damage_type: DamageType::Unknown,
                    attributes: Attrs::Undefined
                }}"
            );

            let base_declaration = format!(
                "pub static {name_ssnake}: CachedRune = CachedRune {{
                    name: {name:?},
                    damage_type: DamageType::Unknown,
                    melee_damage:zero,ranged_damage:zero,
                    riot_id: {riot_id},
                    internal_id: RuneId::{name_pascal},
                    undeclared: true,
                    metadata: {metadata},
                }};"
            );

            let html_declaration =
                base_declaration.replace("melee_damage:zero,ranged_damage:zero", "damage: zero");

            println!("[build] RuneId::{name_pascal}");

            runes.push(RuneResult {
                name_pascal,
                deals_damage: false,
                match_arm: "zero(ctx)".into(),
                riot_id,
                html_declaration,
                base_declaration,
                name_ssnake,
                html_closure: String::new(),
                name,
                idents: "&[]".into(),
            });
        }

        runes.sort_by(|a, b| a.name.cmp(&b.name));
        runes
    };

    build_runes(data)
}

fn build_runes(data: Vec<RuneResult>) -> GeneratorFn {
    let len = data.len();
    let mut rune_declarations = String::new();

    let [
        mut rune_cache,
        mut rune_formulas,
        mut rune_idents,
        mut rune_closures,
    ] = std::array::from_fn(|i| {
        let (name, vtype) = [
            ("RUNE_CACHE", "&CachedRune"),
            ("RUNE_FORMULAS", "Range<usize>"),
            ("RUNE_IDENTS", "&[CtxVar]"),
            ("RUNE_CLOSURES", "Range<usize>"),
        ][i];
        format!("pub static {name}: [{vtype}; RuneId::VARIANTS] = [")
    });

    let mut block = String::new();

    let mut tracker = Tracker::new(&mut block);

    let mut formula_offsets = Vec::with_capacity(len);
    let mut closure_offsets = Vec::with_capacity(len);

    let mut rune_id_enum_match_arms = Vec::with_capacity(len);
    let mut rune_id_enum_fields = Vec::with_capacity(len);
    let mut const_match_arms = String::new();
    let mut rustfmt_inputs = Vec::with_capacity(len * 2);

    for RuneResult {
        riot_id,
        deals_damage,
        html_declaration,
        base_declaration,
        match_arm,
        name_ssnake,
        name_pascal,
        idents,
        html_closure,
        ..
    } in data
    {
        if deals_damage {
            let match_arm = format!("RuneId::{name_pascal} => {{{match_arm}}}");
            const_match_arms.push_str(&match_arm);
        }

        rune_idents.push_str(&format!("{idents},"));
        rune_id_enum_match_arms.push(format!("{riot_id} => Some(Self::{name_pascal})"));
        rune_id_enum_fields.push(name_pascal);
        rune_cache.push_str(&format!("&{name_ssnake},"));
        rune_declarations.push_str(&base_declaration);
        rustfmt_inputs.push(html_declaration);
        rustfmt_inputs.push(html_closure);
    }

    let formatted = rustfmt_batch(&rustfmt_inputs);

    for i in 0..len {
        let decl_fmt = &formatted[i * 2];
        let clos_fmt = &formatted[i * 2 + 1];
        let html_declaration = decl_fmt.rust_html().as_const();
        tracker.record_into(&html_declaration, &mut formula_offsets);
        match clos_fmt.trim().is_empty() {
            true => closure_offsets.push(MAX_TUPLE),
            false => tracker.record_into(&clos_fmt.drop_f32s().rust_html(), &mut closure_offsets),
        }
    }

    let fields = rune_id_enum_fields.join(",");
    let match_arms = rune_id_enum_match_arms.join(",");

    let rune_id_enum = format!(
        r#"
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[derive(bincode::Encode, bincode::Decode)]
        #[derive(serde::Serialize, serde::Deserialize)]
        #[repr(u8)]
        pub enum RuneId {{ {fields} }}
        impl RuneId {{
            pub const VARIANTS: usize = {len};
            pub const fn from_riot_id(id: u32) -> Option<Self> {{
                match id {{ {match_arms}, _ => None }}
            }}
        }}"#
    );

    push_end([&mut rune_cache, &mut rune_idents], "];");

    let const_eval = format!(
        "pub const fn rune_const_eval(
            ctx: &Ctx, 
            rune_id: RuneId, 
            attack_type: AttackType
        ) -> f32 {{
            match rune_id {{ {const_match_arms}, _ => 0.0 }}
        }}"
    );

    println!("[ok] Finished building runes");

    let callback = move |index: usize| {
        let add_offsets = |list: Vec<_>, target: &mut String| {
            for tuple in list {
                match tuple {
                    MAX_TUPLE => {
                        let (s, e) = unsafe { ZERO_FN_OFFSET };
                        target.push_str(&format!("({s}..{e}),"));
                    }
                    (start, end) => {
                        let new_start = start + index;
                        let new_end = end + index;
                        target.push_str(&format!("({new_start}..{new_end}),"));
                    }
                }
            }
            push_end([target], "];");
        };

        add_offsets(formula_offsets, &mut rune_formulas);
        add_offsets(closure_offsets, &mut rune_closures);

        let content = [
            rune_cache,
            rune_declarations,
            rune_id_enum,
            rune_formulas,
            rune_closures,
            rune_idents,
            const_eval,
        ]
        .concat();

        let exports = format!("pub mod runes {{ use super::*; {content} }}");

        Generated { exports, block }
    };

    Ok(Box::new(callback))
}
