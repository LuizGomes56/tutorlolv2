use crate::{
    CwdPath, Generated, GeneratorFn, SrcFolder, Tracker, push_end,
    scripts::{StringExt, USE_SUPER, model::Rune},
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::collections::HashMap;

struct RuneResult {
    name: String,
    name_pascal: String,
    name_ssnake: String,
    declaration: String,
    formula: String,
    riot_id: usize,
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

                println!("Building: Rune({name:?})");

                let name_pascal = name.pascal_case();
                let name_ssnake = name.to_ssnake();

                let metadata = format!(
                    "TypeMetadata {{
                        kind: RuneId::{name_pascal},
                        damage_type: DamageType::{damage_type},
                        attributes: Attrs::Undefined
                    }}"
                );

                let get_closure = |expr: String| {
                    let closure = expr.as_closure().add_f32s();
                    let arg = closure.ctx_param();
                    let body = closure.to_lowercase();
                    format!("|{arg}| {body}")
                };

                let melee_closure = get_closure(ranged);
                let range_closure = get_closure(melee);

                let declaration = format!(
                    "pub static {name_ssnake}_{riot_id}: CachedRune = CachedRune {{
                        damage_type: DamageType::{damage_type},
                        metadata: {metadata},
                        melee_closure: {melee_closure},
                        range_closure: {range_closure},
                        riot_id: {riot_id},
                        undeclared: false
                    }};"
                );

                RuneResult {
                    riot_id,
                    formula: declaration.rust_fmt(80).drop_f32s().rust_html().as_const(),
                    declaration,
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

            let declaration = format!(
                "pub static {name_ssnake}_{riot_id}: CachedRune = CachedRune {{
                    damage_type: DamageType::Unknown,
                    metadata: {metadata},
                    melee_closure: zero,
                    range_closure: zero,
                    riot_id: {riot_id},
                    undeclared: true
                }};"
            );

            runes.push(RuneResult {
                name_pascal,
                riot_id,
                formula: declaration.rust_fmt(80).drop_f32s().rust_html().as_const(),
                declaration,
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
        let (name, vtype) = [
            ("RUNE_CACHE", "&CachedRune"),
            ("RUNE_ID_TO_NAME", "&str"),
            ("RUNE_FORMULAS", "(u32,u32)"),
            ("RUNE_ID_TO_RIOT_ID", "u32"),
        ][i];
        format!("pub static {name}: [{vtype}; {len}] = [")
    });

    let mut block = String::new();

    let mut tracker = Tracker::new(&mut block);
    let mut formula_offsets = Vec::with_capacity(len);

    let mut rune_id_enum_match_arms = Vec::new();
    let mut rune_id_enum_fields = Vec::new();

    for RuneResult {
        riot_id,
        formula,
        declaration,
        name,
        name_ssnake,
        name_pascal,
    } in data
    {
        rune_id_to_riot_id.push_str(&format!("{riot_id},"));
        rune_id_enum_match_arms.push(format!("{riot_id} => Some(Self::{name_pascal})"));
        rune_id_enum_fields.push(name_pascal);
        rune_id_to_name.push_str(&format!("{name:?},"));
        rune_cache.push_str(&format!("&{name_ssnake}_{riot_id},"));
        rune_declarations.push_str(&declaration);
        tracker.record_into(&formula, &mut formula_offsets);
    }

    let fields = rune_id_enum_fields.join(",");
    let match_arms = rune_id_enum_match_arms.join(",");

    let rune_id_enum = format!(
        "#[derive(Debug, Copy, Clone, Ord, Eq, PartialOrd, PartialEq, Decode, Encode)]
        #[repr(u8)]
        pub enum RuneId {{ {fields} }}
        impl RuneId {{
            pub const fn to_riot_id(&self) -> u32 {{
                RUNE_CACHE[*self as usize].riot_id
            }}
            pub const fn from_riot_id(id: u32) -> Option<Self> {{
                match id {{ {match_arms}, _ => None }}
            }}
            pub const unsafe fn from_u8_unchecked(id: u8) -> Self {{
                unsafe {{ core::mem::transmute(id) }}
            }}
            pub const fn from_u8(id: u8) -> Option<Self> {{
                if id < {len} as u8 {{
                    Some(unsafe {{ Self::from_u8_unchecked(id) }})
                }} else {{
                    None
                }}
            }}
        }}"
    );

    push_end(
        [
            &mut rune_cache,
            &mut rune_id_to_name,
            &mut rune_id_to_riot_id,
        ],
        "];",
    );

    let imports = [USE_SUPER, &rune_cache, &rune_id_enum, &rune_declarations].concat();
    CwdPath::fwrite(SrcFolder::Runes.import_file(), imports).await??;

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

        let exports = [
            rune_id_enum.replace(
                "RUNE_CACHE[*self as usize].riot_id",
                "RUNE_ID_TO_RIOT_ID[*self as usize]",
            ),
            rune_id_to_name,
            rune_formulas,
            rune_id_to_riot_id,
        ]
        .concat();

        Generated { exports, block }
    };

    Ok(Box::new(callback))
}
