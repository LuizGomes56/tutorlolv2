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
    constfn_declaration: String,
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

    let name = name.pascal_case();

    let metadata = format!(
        "TypeMetadata {{ 
            kind: ItemId::{name}, 
            damage_type: DamageType::{damage_type}, 
            attributes: Attrs::{attributes:?} 
        }}"
    );

    let get_closure = |expr: &str| match expr {
        expr if expr.is_empty() => {
            let new_expr = expr.as_closure().add_f32s();
            let ctx_param = new_expr.ctx_param();
            let body = new_expr.to_lowercase();
            format!("|{ctx_param}| {body}")
        }
        zero => zero.to_string(),
    };

    let mut constfn_declaration = String::new();

    // let mut make_closure = |damage_object: &DamageObject, tag| {
    //     [
    //         (
    //             "minimum_damage",
    //             assign_value(&damage_object.minimum_damage),
    //         ),
    //         (
    //             "maximum_damage",
    //             assign_value(&damage_object.maximum_damage),
    //         ),
    //     ]
    //     .into_iter()
    //     .map(|(rtype, closure)| {
    //         let closure = closure.unwrap_or("zero".to_string());
    //         let body = closure.trim_start_matches("|ctx|");
    //         let fn_name = format!("{name}_{tag}_{rtype}").to_lowercase();
    //         constfn_declaration.push_str(&format!(
    //             "pub const fn {fn_name}(ctx: &EvalContext) -> f32 {{ {body} }}",
    //         ));
    //     })
    // };

    // let range_closure = make_closure(&ranged, "range");
    // let melee_closure = make_closure(&melee, "melee");

    todo!();

    // DeclaredItem {
    //     metadata,
    //     range_closure,
    //     melee_closure,
    //     constfn_declaration,
    // }
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
        html_declaration: String,
        base_declaration: String,
        name: String,
        name_ssnake: String,
        name_pascal: String,
        generator: String,
    }

    let mut data = parallel_task(
        128,
        "internal/items",
        async |file_name, item: Item| {
            println!("[build] ItemId({file_name:?})");

            let DeclaredItem {
                metadata,
                range_closure,
                melee_closure,
                constfn_declaration,
            } = declare_item(&item);

            let Item {
                riot_id,
                name,
                prettified_stats,
                tier,
                price,
                purchasable,
                damage_type,
                stats,
                ranged,
                melee,
                attributes,
            } = item;

            let name_ssnake = name.to_ssnake();
            let name_pascal = name.pascal_case();
            let prettified_stats = prettified_stats
                .iter()
                .map(|stat| format!("StatName::{stat:?}"))
                .collect::<Vec<_>>()
                .join(",");

            let stats = format_stats(&stats);
            let is_simulated = tier >= 3 && price > 0 && purchasable;
            let is_damaging = {
                let is_zeroed = |expr: &str| expr != "zero" && !expr.is_empty();
                is_zeroed(&ranged.minimum_damage)
                    || is_zeroed(&ranged.maximum_damage)
                    || is_zeroed(&melee.minimum_damage)
                    || is_zeroed(&melee.maximum_damage)
            };

            let mut base_declaration = format!(
                "pub static {name_ssnake}_{riot_id}: CachedItem = CachedItem {{
                    gold: {price},
                    prettified_stats: &[{prettified_stats}],
                    damage_type: DamageType::{damage_type},
                    attributes: Attrs::{attributes:?},
                    metadata: {metadata},
                    stats: CachedItemStats {{ {stats} }},
                    is_simulated: {is_simulated},
                    is_damaging: {is_damaging},
                    riot_id: {riot_id},"
            );

            let html_declaration = format!(
                "{base_declaration}
                range_closure: {range_closure},
                melee_closure: {melee_closure}, }};"
            )
            .rust_fmt(80)
            .drop_f32s()
            .rust_html()
            .as_const();

            base_declaration.push_str(&format!(
                "range_closure: {name_pascal}_range,
                melee_closure: {name_pascal}_melee, }};"
            ));

            base_declaration.push_str(&constfn_declaration);

            let generator =
                CwdPath::get_generator(SrcFolder::Items, name_ssnake.to_lowercase()).await?;

            Ok(ItemResult {
                riot_id,
                html_declaration,
                base_declaration,
                generator,
                name_ssnake,
                name_pascal,
                name,
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
        html_declaration: formula,
        base_declaration: declaration,
        name,
        name_ssnake,
        name_pascal,
        generator,
    } in data
    {
        item_id_to_riot_id.push_str(&format!("{riot_id},"));
        item_id_enum_match_arms.push(format!("{riot_id} => Some(Self::{name_pascal})"));
        item_id_enum_fields.push(name_pascal);
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

    CwdPath::fwrite(
        SrcFolder::Items.import_file(),
        [USE_SUPER, &item_cache, &item_id_enum, &item_declarations]
            .concat()
            .rust_fmt(80),
    )
    .await??;

    let callback = move |index: usize| {
        let add_offsets = |(list, target): (Vec<_>, &mut String)| {
            for (start, end) in list {
                let new_start = start + index;
                let new_end = end + index;
                target.push_str(&format!("({new_start}, {new_end}),"));
            }
            push_end([target], "];");
        };

        [
            (generator_offsets, &mut item_generator),
            (formula_offsets, &mut item_formulas),
        ]
        .into_iter()
        .for_each(add_offsets);

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
