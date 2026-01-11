use crate::{
    CwdPath, EVAL_FEAT, GLOB_FEAT, Generated, GeneratorFn, SrcFolder, Tracker, parallel_task,
    push_end,
    scripts::{
        StringExt,
        model::{DamageObject, Item, ItemStats, MerakiItemStatMap},
    },
};

struct DeclaredItem {
    metadata: String,
    ranged_closures: String,
    melee_closures: String,
    constfn_declaration: String,
    melee_fn_names: String,
    ranged_fn_names: String,
    match_arm: String,
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
    let name_ssnake = name.to_ssnake();

    let metadata = format!(
        "TypeMetadata {{ 
            kind: ItemId::{name}, 
            damage_type: DamageType::{damage_type}, 
            attributes: Attrs::{attributes:?} 
        }}"
    );

    let mut constfn_declaration = String::new();

    let get_constfn_closure = |expr: &str| {
        (!expr.is_empty() && expr != "zero").then_some({
            let ctx_param = expr.ctx_param();
            let body = expr.to_lowercase();
            let closure = format!("|{ctx_param}| {body}");
            move |fn_name: &str| {
                (
                    format!(
                        "{EVAL_FEAT} pub const fn {fn_name}(ctx: &EvalContext) -> f32 {{ {expr} }}",
                        expr = body.clean().cast_f32()
                    ),
                    closure,
                )
            }
        })
    };

    let mut melee_fn_names = Vec::with_capacity(2);
    let mut ranged_fn_names = Vec::with_capacity(2);

    let mut get_closure = |obj: &DamageObject, fn_vec: &mut Vec<_>, tag| {
        let values = [&obj.minimum_damage, &obj.maximum_damage];
        let dtypes = ["min", "max"];
        std::array::from_fn::<_, 2, _>(|i| {
            let (fn_name, closure) = match get_constfn_closure(values[i]) {
                Some(f) => {
                    let dtype = dtypes[i];
                    let fn_name = format!("{name_ssnake}_{tag}_{dtype}").to_lowercase();
                    let (constfn, closure) = f(&fn_name);
                    constfn_declaration.push_str(&constfn);
                    (fn_name, closure)
                }
                None => ("zero".into(), "zero".into()),
            };
            fn_vec.push(fn_name);
            closure
        })
    };

    let ranged_closures = {
        let [min, max] = get_closure(&ranged, &mut ranged_fn_names, "ranged");
        format!("ranged_min_dmg: {min}, ranged_max_dmg: {max},")
    };
    let melee_closures = {
        let [min, max] = get_closure(&melee, &mut melee_fn_names, "melee");
        format!("melee_min_dmg: {min}, melee_max_dmg: {max},")
    };

    let get_fn_call = |names: &[String]| {
        names
            .iter()
            .map(|fn_name| format!("{fn_name}(ctx)"))
            .collect::<Vec<_>>()
            .join(",")
    };

    let match_arm = format!(
        "AttackType::Melee => [{melee_fn_calls}],
        AttackType::Ranged => [{ranged_fn_calls}]",
        melee_fn_calls = get_fn_call(&melee_fn_names),
        ranged_fn_calls = get_fn_call(&ranged_fn_names),
    );

    DeclaredItem {
        metadata,
        match_arm,
        ranged_closures,
        melee_closures,
        melee_fn_names: melee_fn_names.join(","),
        ranged_fn_names: ranged_fn_names.join(","),
        constfn_declaration,
    }
}

pub fn get_stats(stats: &ItemStats) -> String {
    const NUMBER_OF_STATS: usize = size_of::<ItemStats>() / size_of::<MerakiItemStatMap>() + 2;
    let mut all_stats = Vec::with_capacity(NUMBER_OF_STATS);

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

struct ItemResult {
    riot_id: usize,
    html_declaration: String,
    base_declaration: String,
    name: String,
    name_ssnake: String,
    name_pascal: String,
    generator: String,
    match_arm: String,
    idents: String,
}

pub fn generate_items() -> GeneratorFn {
    let mut data = parallel_task("internal/items", |_, item: Item| {
        let DeclaredItem {
            metadata,
            match_arm,
            ranged_closures,
            melee_closures,
            melee_fn_names,
            ranged_fn_names,
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

        println!("[build] ItemId::{name_pascal}");

        let prettified_stats = prettified_stats
            .iter()
            .map(|stat| format!("StatName::{stat:?}"))
            .collect::<Vec<_>>()
            .join(",");

        let stats = get_stats(&stats);
        let deals_damage = {
            let is_zeroed = |expr: &str| expr != "zero" && !expr.is_empty();
            is_zeroed(&ranged.minimum_damage)
                || is_zeroed(&ranged.maximum_damage)
                || is_zeroed(&melee.minimum_damage)
                || is_zeroed(&melee.maximum_damage)
        };

        let rest = format!(
            "internal_id: ItemId::{name_pascal},
                riot_id: {riot_id},
                deals_damage: {deals_damage},
                stats: CachedItemStats {{ {stats} }},
                metadata: {metadata} }};"
        );

        let base_declaration = format!(
            "pub static {name_ssnake}_{riot_id}: CachedItem = CachedItem {{
                name: {name:?},
                price: {price},
                prettified_stats: &[{prettified_stats}],
                damage_type: DamageType::{damage_type},
                attributes: Attrs::{attributes:?},
                tier: {tier},
                purchasable: {purchasable},"
        );

        let html_declaration = format!("{base_declaration}{ranged_closures}{melee_closures}{rest}")
            .rust_fmt()
            .drop_f32s()
            .rust_html()
            .as_const();

        let base_declaration = format!(
            "{EVAL_FEAT}{base_declaration}
                ranged_damages: [{ranged_fn_names}],
                melee_damages: [{melee_fn_names}], {rest}
                {constfn_declaration}"
        );

        let generator = CwdPath::get_generator(SrcFolder::Items, name_ssnake.to_lowercase())?;

        let idents = (melee_closures + &ranged_closures)
            .get_idents()
            .into_iter()
            .collect::<String>();

        Ok(ItemResult {
            riot_id,
            match_arm,
            html_declaration,
            base_declaration,
            generator,
            name_ssnake,
            name_pascal,
            name,
            idents: format!("&[{idents}]"),
        })
    });

    data.sort_by(|a, b| a.1.name.cmp(&b.1.name));
    build_items(data)
}

fn build_items(data: Vec<(String, ItemResult)>) -> GeneratorFn {
    let len = data.len();
    let mut item_declarations = String::new();

    let [
        mut item_cache,
        mut item_id_to_name,
        mut item_formulas,
        mut item_generator,
        mut item_id_to_riot_id,
        mut item_idents,
    ] = std::array::from_fn(|i| {
        let (name, vtype, feature) = [
            ("ITEM_CACHE", "&CachedItem", EVAL_FEAT),
            ("ITEM_ID_TO_NAME", "&str", GLOB_FEAT),
            ("ITEM_FORMULAS", "(u32,u32)", GLOB_FEAT),
            ("ITEM_GENERATOR", "(u32,u32)", GLOB_FEAT),
            ("ITEM_ID_TO_RIOT_ID", "u32", GLOB_FEAT),
            ("ITEM_IDENTS", "&[EvalIdent]", GLOB_FEAT),
        ][i];
        format!("{feature} pub static {name}: [{vtype}; ItemId::VARIANTS] = [")
    });

    let mut block = String::new();

    let mut tracker = Tracker::new(&mut block);
    let mut formula_offsets = Vec::with_capacity(len);
    let mut generator_offsets = Vec::with_capacity(len);

    let mut item_id_enum_match_arms = Vec::new();
    let mut item_id_enum_fields = Vec::new();
    let mut const_match_arms = String::new();

    for (
        _,
        ItemResult {
            riot_id,
            match_arm,
            html_declaration,
            base_declaration,
            name,
            name_ssnake,
            name_pascal,
            generator,
            idents,
        },
    ) in data
    {
        let match_arm = format!(
            "ItemId::{name_pascal} => {{ 
                match attack_type {{ {match_arm} }} 
            }}"
        );

        item_idents.push_str(&(idents + ","));
        const_match_arms.push_str(&match_arm);
        item_id_to_riot_id.push_str(&format!("{riot_id},"));
        item_id_enum_match_arms.push({
            let str_id = riot_id.to_string();
            let mut branches = Vec::with_capacity(3);
            if !(str_id.starts_with("22") || str_id.starts_with("44")) {
                branches.push(format!("44{riot_id}"));
                branches.push(format!("22{riot_id}"));
            }
            branches.push(str_id);
            branches.reverse();
            let branches = branches.join("|");
            format!("{branches} => Some(Self::{name_pascal})")
        });
        item_id_enum_fields.push(name_pascal);
        item_id_to_name.push_str(&format!("{name:?},"));
        item_cache.push_str(&format!("&{name_ssnake}_{riot_id},"));
        item_declarations.push_str(&base_declaration);
        tracker.record_into(&generator, &mut generator_offsets);
        tracker.record_into(&html_declaration, &mut formula_offsets);
    }

    let fields = item_id_enum_fields.join(",");
    let match_arms = item_id_enum_match_arms.join(",");

    let item_id_enum = format!(
        r#"
        #[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
        #[derive(bincode::Encode, bincode::Decode)]
        #[derive(serde::Serialize, serde::Deserialize)]
        #[repr(u16)]
        pub enum ItemId {{ {fields} }}
        impl ItemId {{
            pub const VARIANTS: usize = {len};
            {EVAL_FEAT}
            pub const fn to_riot_id(&self) -> u32 {{
                ITEM_CACHE[*self as usize].riot_id
            }}
            {EVAL_FEAT}
            pub const fn from_riot_id(id: u32) -> Option<Self> {{
                match id {{ {match_arms}, _ => None }}
            }}
            pub const unsafe fn from_u16_unchecked(id: u16) -> Self {{
                unsafe {{ core::mem::transmute(id) }}
            }}
            pub const fn from_u16(id: u16) -> Option<Self> {{
                if id < Self::VARIANTS as u16 {{
                    Some(unsafe {{ Self::from_u16_unchecked(id) }})
                }} else {{
                    None
                }}
            }}
        }}"#
    );

    let const_eval = format!(
        "{EVAL_FEAT} pub const fn item_const_eval(
            ctx: &EvalContext, 
            item_id: ItemId, 
            attack_type: AttackType
        ) -> [f32; 2] {{
            match item_id {{ {const_match_arms} }}
        }}"
    );

    push_end(
        [
            &mut item_cache,
            &mut item_id_to_name,
            &mut item_id_to_riot_id,
            &mut item_idents,
        ],
        "];",
    );

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

        let content = [
            item_id_enum,
            item_id_to_name,
            item_formulas,
            item_id_to_riot_id,
            item_generator,
            item_cache,
            item_declarations,
            item_idents,
            const_eval,
        ]
        .concat()
        .rust_fmt();

        let exports = format!("pub mod items {{ use super::*; {content} }}");

        Generated { exports, block }
    };

    Ok(Box::new(callback))
}
