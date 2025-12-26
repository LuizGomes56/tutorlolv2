use crate::{
    CwdPath, EVAL_FEAT, GLOB_FEAT, Generated, GeneratorFn, Tracker, push_end,
    scripts::{StringExt, model::Rune},
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::collections::HashMap;

struct RuneResult {
    name: String,
    name_pascal: String,
    name_ssnake: String,
    base_declaration: String,
    html_declaration: String,
    riot_id: usize,
    match_arm: String,
}

pub async fn generate_runes() -> GeneratorFn {
    let data = {
        let json = CwdPath::deserialize::<HashMap<usize, Rune>>("internal/runes.json").await?;

        let mut runes = json
            .into_par_iter()
            .map(|(riot_id, rune)| {
                let Rune {
                    name,
                    damage_type,
                    ranged,
                    melee,
                } = rune;

                println!("[build] Rune({name:?})");

                let name_pascal = name.pascal_case();
                let name_ssnake = name.to_ssnake();

                let metadata = format!(
                    "TypeMetadata {{
                        kind: RuneId::{name_pascal},
                        damage_type: DamageType::{damage_type},
                        attributes: Attrs::Undefined
                    }}"
                );

                let mut constfn_declaration = String::new();

                let mut get_closure = |expr: String, tag| {
                    let closure = expr.as_closure().add_f32s();
                    let arg = closure.ctx_param();
                    let body = closure.to_lowercase();
                    let fn_name = format!("{name_ssnake}_{tag}").to_lowercase();
                    constfn_declaration.push_str(&format!(
                        "{EVAL_FEAT} pub const fn {fn_name}(ctx: &EvalContext) -> f32 {{ {body} }}"
                    ));
                    format!("|{arg}| {body}")
                };

                let melee_closure = get_closure(ranged, "ranged");
                let ranged_closure = get_closure(melee, "melee");

                let base_declaration = format!(
                    "pub static {name_ssnake}_{riot_id}: CachedRune = CachedRune {{
                        name: {name:?},
                        damage_type: DamageType::{damage_type},
                        metadata: {metadata},
                        riot_id: {riot_id},
                        internal_id: RuneId::{name_pascal},
                        undeclared: false,"
                );

                let html_declaration = format!(
                    "{base_declaration}
                    melee_damage: {melee_closure},
                    ranged_damage: {ranged_closure} }};"
                )
                .rust_fmt()
                .drop_f32s()
                .rust_html()
                .as_const();

                let melee_constfn_name = format!("{name_ssnake}_melee").to_lowercase();
                let ranged_constfn_name = format!("{name_ssnake}_ranged").to_lowercase();

                let base_declaration = format!(
                    "{EVAL_FEAT}{base_declaration}
                    melee_damage: {melee_constfn_name},
                    ranged_damage: {ranged_constfn_name} }};
                    {constfn_declaration}"
                );

                let match_arm = format!(
                    "AttackType::Melee => {melee_constfn_name}(ctx),
                    AttackType::Ranged => {ranged_constfn_name}(ctx)",
                );

                RuneResult {
                    riot_id,
                    match_arm,
                    html_declaration,
                    base_declaration,
                    name,
                    name_ssnake,
                    name_pascal,
                }
            })
            .collect::<Vec<_>>();

        let mut undeclared =
            CwdPath::deserialize::<HashMap<String, usize>>("internal/rune_names.json").await?;

        for (name, riot_id) in [
            ("Adaptive Force", 9990),
            ("Health Scaling", 9991),
            ("Attack Speed", 9992),
            ("Health", 9993),
            ("Ability Haste", 9994),
            ("Tenacity and Slow Resist", 9995),
            ("Move Speed", 9996),
            ("Eyeball Collection", 8120),
            ("Ghost Poro", 8136),
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
                "{EVAL_FEAT} pub static {name_ssnake}_{riot_id}: CachedRune = CachedRune {{
                    name: {name:?},
                    damage_type: DamageType::Unknown,
                    metadata: {metadata},
                    melee_damage: zero,
                    ranged_damage: zero,
                    riot_id: {riot_id},
                    internal_id: RuneId::{name_pascal},
                    undeclared: true
                }};"
            );

            let match_arm = format!(
                "AttackType::Melee => zero(ctx),
                AttackType::Ranged => zero(ctx)",
            );

            runes.push(RuneResult {
                name_pascal,
                match_arm,
                riot_id,
                html_declaration: base_declaration
                    .rust_fmt()
                    .drop_f32s()
                    .rust_html()
                    .as_const(),
                base_declaration,
                name_ssnake,
                name,
            });
        }

        runes.sort_by(|a, b| a.name.cmp(&b.name));
        runes
    };

    let len = data.len();
    let mut rune_declarations = String::new();

    let [
        mut rune_cache,
        mut rune_id_to_name,
        mut rune_formulas,
        mut rune_id_to_riot_id,
    ] = std::array::from_fn(|i| {
        let (name, vtype, feature) = [
            ("RUNE_CACHE", "&CachedRune", EVAL_FEAT),
            ("RUNE_ID_TO_NAME", "&str", GLOB_FEAT),
            ("RUNE_FORMULAS", "(u32,u32)", GLOB_FEAT),
            ("RUNE_ID_TO_RIOT_ID", "u32", GLOB_FEAT),
        ][i];
        format!("{feature} pub static {name}: [{vtype}; RuneId::VARIANTS] = [")
    });

    let mut block = String::new();

    let mut tracker = Tracker::new(&mut block);
    let mut formula_offsets = Vec::with_capacity(len);

    let mut rune_id_enum_match_arms = Vec::new();
    let mut rune_id_enum_fields = Vec::new();
    let mut const_match_arms = String::new();

    for RuneResult {
        riot_id,
        html_declaration,
        base_declaration,
        match_arm,
        name,
        name_ssnake,
        name_pascal,
    } in data
    {
        let match_arm = format!(
            "RuneId::{name_pascal} => {{ 
                match attack_type {{ {match_arm} }} 
            }}"
        );

        const_match_arms.push_str(&match_arm);
        rune_id_to_riot_id.push_str(&format!("{riot_id},"));
        rune_id_enum_match_arms.push(format!("{riot_id} => Some(Self::{name_pascal})"));
        rune_id_enum_fields.push(name_pascal);
        rune_id_to_name.push_str(&format!("{name:?},"));
        rune_cache.push_str(&format!("&{name_ssnake}_{riot_id},"));
        rune_declarations.push_str(&base_declaration);
        tracker.record_into(&html_declaration, &mut formula_offsets);
    }

    let fields = rune_id_enum_fields.join(",");
    let match_arms = rune_id_enum_match_arms.join(",");

    let rune_id_enum = format!(
        r#"
        #[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
        #[derive(bincode::Encode, bincode::Decode)]
        #[derive(serde::Serialize, serde::Deserialize)]
        #[repr(u8)]
        pub enum RuneId {{ {fields} }}
        impl RuneId {{
            pub const VARIANTS: usize = {len};
            {EVAL_FEAT}
            pub const fn to_riot_id(&self) -> u32 {{
                RUNE_CACHE[*self as usize].riot_id
            }}
            {EVAL_FEAT}
            pub const fn from_riot_id(id: u32) -> Option<Self> {{
                match id {{ {match_arms}, _ => None }}
            }}
            pub const unsafe fn from_u8_unchecked(id: u8) -> Self {{
                unsafe {{ core::mem::transmute(id) }}
            }}
            pub const fn from_u8(id: u8) -> Option<Self> {{
                if id < Self::VARIANTS as u8 {{
                    Some(unsafe {{ Self::from_u8_unchecked(id) }})
                }} else {{
                    None
                }}
            }}
        }}"#
    );

    push_end(
        [
            &mut rune_cache,
            &mut rune_id_to_name,
            &mut rune_id_to_riot_id,
        ],
        "];",
    );

    let const_eval = format!(
        "{EVAL_FEAT} pub const fn rune_const_eval(
            ctx: &EvalContext, 
            rune_id: RuneId, 
            attack_type: AttackType
        ) -> f32 {{
            match rune_id {{ {const_match_arms} }}
        }}"
    );

    let callback = move |index: usize| {
        let add_offsets = |list: Vec<_>, target: &mut String| {
            for (start, end) in list {
                let new_start = start + index;
                let new_end = end + index;
                target.push_str(&format!("({new_start}, {new_end}),"));
            }
            push_end([target], "];");
        };

        add_offsets(formula_offsets, &mut rune_formulas);

        let content = [
            rune_cache,
            rune_declarations,
            rune_id_enum,
            rune_id_to_name,
            rune_formulas,
            rune_id_to_riot_id,
            const_eval,
        ]
        .concat()
        .rust_fmt();

        let exports = format!("pub mod runes {{ use super::*; {content} }}");

        Generated { exports, block }
    };

    Ok(Box::new(callback))
}
