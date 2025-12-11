use crate::{
    CwdPath, Generated, GeneratorFn, SrcFolder, Tracker, parallel_task, push_end,
    scripts::{
        StringExt, USE_SUPER,
        model::{DamageObject, Item, ItemStats},
    },
};

struct DeclaredItem {
    metadata: String,
    range_closure: String,
    melee_closure: String,
}

fn declare_item(item: &Item) -> DeclaredItem {
    let Item {
        name,
        damage_type,
        ranged,
        melee,
        attributes,
        ..
    } = item;

    let name = name.normalize();

    let metadata = format!(
        "TypeMetadata {{ 
            kind: ItemId::{name}, 
            damage_type: DamageType::{damage_type}, 
            attributes: Attrs::{attributes:?} 
        }}"
    );

    let assign_value = |expr: &str| {
        (!(expr.is_empty() || expr == "zero")).then_some({
            let new_expr = expr.as_closure().add_f32s();
            let ctx_param = new_expr.ctx_param();
            let body = new_expr.to_lowercase();
            format!("|{ctx_param}| {body}")
        })
    };

    let make_closure = |damage_object: &DamageObject| {
        let mut closures = Vec::new();
        if let Some(min) = assign_value(&damage_object.minimum_damage) {
            closures.push(min);
        };
        if let Some(max) = assign_value(&damage_object.maximum_damage) {
            closures.push(max);
        };
        let data = closures.join(",");
        format!("&[{data}]")
    };

    let range_closure = make_closure(&ranged);
    let melee_closure = make_closure(&melee);

    DeclaredItem {
        metadata,
        range_closure,
        melee_closure,
    }
}

pub fn format_stats(stats: &ItemStats) -> String {
    let mut all_stats = Vec::new();

    macro_rules! insert_stats {
        (
            flat => { $($field:ident),+$(,)* },
            $(pen $disc:ident => { $($pfield:ident),+$(,)* }),+
        ) => {
            $(
                all_stats.push(
                    format!(
                        "{field}: {value}f32",
                        field = stringify!($field),
                        value = stats.$field.flat
                    )
                );
            )+
            $(
                $(
                    all_stats.push(
                        format!(
                            "{field}: {value}f32",
                            field = concat!(
                                stringify!($pfield),
                                "_",
                                stringify!($disc)
                            ),
                            value = stats.$pfield.$disc
                        )
                    );
                )+
            )+
        };
    }

    insert_stats!(
        flat => {
            ability_power,
            armor,
            attack_damage,
            attack_speed,
            crit_chance,
            crit_damage,
            health,
            lifesteal,
            magic_resist,
            mana,
            movespeed,
            omnivamp,
        },
        pen flat => {
            armor_penetration,
            magic_penetration,
        },
        pen percent => {
            armor_penetration,
            magic_penetration,
        }
    );
    all_stats.join(",")
}

pub async fn generate_items() -> GeneratorFn {
    struct ItemResult {
        riot_id: usize,
        formula: String,
        declaration: String,
        name: String,
        name_ssnake: String,
        name_normalized: String,
        generator: String,
    }

    let mut data = parallel_task(
        128,
        "internal/items",
        async |file_name, item: Item| {
            println!("Building: ItemId({file_name:?})");

            let Item {
                riot_id,
                ref name,
                ref prettified_stats,
                tier,
                price,
                purchasable,
                ref damage_type,
                ref stats,
                ref ranged,
                ref melee,
                attributes,
            } = item;

            let name_ssnake = name.to_ssnake();
            let name_normalized = name.normalize();
            let prettified_stats = prettified_stats
                .iter()
                .map(|stat| format!("StatName::{stat:?}"))
                .collect::<Vec<_>>()
                .join(",");

            let DeclaredItem {
                metadata,
                range_closure,
                melee_closure,
            } = declare_item(&item);

            let stats = format_stats(&stats);
            let is_simulated = tier >= 3 && price > 0 && purchasable;
            let is_damaging = {
                let is_zeroed = |expr: &str| expr != "zero" && !expr.is_empty();
                is_zeroed(&ranged.minimum_damage)
                    || is_zeroed(&ranged.maximum_damage)
                    || is_zeroed(&melee.minimum_damage)
                    || is_zeroed(&melee.maximum_damage)
            };

            let declaration = format!(
                "pub static {name_ssnake}_{riot_id}: CachedItem = CachedItem {{
                    gold: {price},
                    prettified_stats: &[{prettified_stats}],
                    damage_type: DamageType::{damage_type},
                    attributes: Attrs::{attributes:?},
                    metadata: {metadata},
                    range_closure: {range_closure},
                    melee_closure: {melee_closure},
                    stats: CachedItemStats {{ {stats} }},
                    is_simulated: {is_simulated},
                    is_damaging: {is_damaging},
                    riot_id: {riot_id},
                }};"
            );

            let generator =
                CwdPath::get_generator(SrcFolder::Items, name_ssnake.to_lowercase()).await?;

            Ok(ItemResult {
                riot_id,
                formula: declaration.rust_fmt(70).drop_f32s().rust_html().as_const(),
                declaration,
                generator,
                name_ssnake,
                name_normalized,
                name: item.name,
            })
        },
        async |futures| {
            let mut result = Vec::new();
            for future in futures {
                let (_, data) = future.await?;
                result.push(data);
            }
            Ok(result)
        },
    )
    .await?;

    data.sort_by(|a, b| a.name.cmp(&b.name));

    let len = data.len();
    let mut item_declarations = String::new();

    let [
        mut item_cache,
        mut item_id_to_name,
        mut item_formulas,
        mut item_generator,
        mut item_id_to_riot_id,
    ] = std::array::from_fn(|i| {
        let (name, vtype) = [
            ("ITEM_CACHE", "&CachedItem"),
            ("ITEM_ID_TO_NAME", "&str"),
            ("ITEM_FORMULAS", "(u32,u32)"),
            ("ITEM_GENERATOR", "(u32,u32)"),
            ("ITEM_ID_TO_RIOT_ID", "u32"),
        ][i];
        format!("pub static {name}: [{vtype}; {len}] = [")
    });

    let mut block = String::new();

    let mut tracker = Tracker::new(&mut block);
    let mut formula_offsets = Vec::with_capacity(len);
    let mut generator_offsets = Vec::with_capacity(len);

    let mut item_id_enum_match_arms = Vec::new();
    let mut item_id_enum_fields = Vec::new();

    for ItemResult {
        riot_id,
        formula,
        declaration,
        name,
        name_ssnake,
        name_normalized,
        generator,
    } in data
    {
        item_id_to_riot_id.push_str(&format!("{riot_id},"));
        item_id_enum_match_arms.push(format!("{riot_id} => Some(Self::{name_normalized})"));
        item_id_enum_fields.push(name_normalized);
        item_id_to_name.push_str(&format!("{name:?},"));
        item_cache.push_str(&format!("&{name_ssnake}_{riot_id},"));
        item_declarations.push_str(&declaration);
        tracker.record_into(&generator, &mut generator_offsets);
        tracker.record_into(&formula, &mut formula_offsets);
    }

    let fields = item_id_enum_fields.join(",");
    let match_arms = item_id_enum_match_arms.join(",");

    let item_id_enum = format!(
        "#[derive(Serialize, Deserialize, Debug, Copy, Clone, Ord, Eq, PartialOrd, PartialEq, Decode, Encode)]
        #[repr(u16)]
        pub enum ItemId {{ {fields} }}
        impl ItemId {{
            pub const fn to_riot_id(&self) -> u32 {{
                ITEM_CACHE[*self as usize].riot_id
            }}
            pub const fn from_riot_id(id: u32) -> Option<Self> {{
                match id {{ {match_arms}, _ => None }}
            }}
            pub const unsafe fn from_u16_unchecked(id: u16) -> Self {{
                unsafe {{ core::mem::transmute(id) }}
            }}
            pub const fn from_u16(id: u16) -> Option<Self> {{
                if id < {len} as u16 {{
                    Some(unsafe {{ Self::from_u16_unchecked(id) }})
                }} else {{
                    None
                }}
            }}
        }}"
    );

    push_end(
        [
            &mut item_cache,
            &mut item_id_to_name,
            &mut item_id_to_riot_id,
        ],
        "];",
    );

    let imports = [USE_SUPER, &item_cache, &item_id_enum, &item_declarations].concat();
    CwdPath::fwrite(SrcFolder::Items.import_file(), imports).await??;

    let callback = move |index: usize| {
        let add_offsets = |list: Vec<_>, target: &mut String| {
            for (start, end) in list {
                let new_start = start + index;
                let new_end = end + index;
                target.push_str(&format!("({new_start}, {new_end}),"));
            }
            push_end([target], "];");
        };

        for (list, target) in [
            (generator_offsets, &mut item_generator),
            (formula_offsets, &mut item_formulas),
        ] {
            add_offsets(list, target);
        }

        let exports = [
            item_id_enum.replace(
                "ITEM_CACHE[*self as usize].riot_id",
                "ITEM_ID_TO_RIOT_ID[*self as usize]",
            ),
            item_id_to_name,
            item_formulas,
            item_id_to_riot_id,
        ]
        .concat();

        Generated { exports, block }
    };

    Ok(Box::new(callback))
}
