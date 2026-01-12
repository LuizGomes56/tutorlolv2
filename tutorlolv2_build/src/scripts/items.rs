use std::collections::{HashMap, HashSet};

use crate::{
    CwdPath, EVAL_FEAT, GLOB_FEAT, Generated, GeneratorFn, SrcFolder, Tracker, parallel_task,
    push_end,
    scripts::{
        StringExt,
        model::{Item, ItemStats, MerakiItemStatMap},
    },
};

struct DeclaredItem {
    metadata: String,
    ranged_closures: [String; 2],
    melee_closures: [String; 2],
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

    #[derive(Clone, Hash, PartialEq, Eq)]
    struct ExprKey {
        body: String,
    }

    #[derive(Clone)]
    struct ExprInfo {
        tag: &'static str,
        kind: &'static str,
        key: ExprKey,
    }

    let normalize = |expr: &str| -> Option<String> {
        (!expr.is_empty() && expr != "zero").then(|| expr.clean().to_lowercase().cast_f32())
    };

    let mut exprs = Vec::<ExprInfo>::new();

    let push_expr = |tag, kind, expr: &str, vec: &mut Vec<ExprInfo>| {
        if let Some(body) = normalize(expr) {
            vec.push(ExprInfo {
                tag,
                kind,
                key: ExprKey { body },
            });
        }
    };

    push_expr("melee", "min", &melee.minimum_damage, &mut exprs);
    push_expr("melee", "max", &melee.maximum_damage, &mut exprs);
    push_expr("ranged", "min", &ranged.minimum_damage, &mut exprs);
    push_expr("ranged", "max", &ranged.maximum_damage, &mut exprs);

    let mut groups: HashMap<ExprKey, Vec<ExprInfo>> = HashMap::new();

    for e in exprs {
        groups.entry(e.key.clone()).or_default().push(e);
    }

    let decide_fn_name = |group: &[ExprInfo]| -> String {
        let kinds = group.iter().map(|e| e.kind).collect::<HashSet<_>>();
        let tags = group.iter().map(|e| e.tag).collect::<HashSet<_>>();

        let name = name_ssnake.to_lowercase();

        let data = &group[0];
        let kind = data.kind;
        let tag = data.tag;

        match (kinds.len(), tags.len()) {
            (2, 2) => name.clone(),
            (1, 2) => format!("{name}_{kind}"),
            (2, 1) => format!("{name}_{tag}"),
            _ => format!("{name}_{tag}_{kind}"),
        }
    };

    let mut constfn_declaration = String::new();
    let mut fn_name_by_slot = HashMap::<(&'static str, &'static str), String>::new();

    for (key, group) in &groups {
        let fn_name = decide_fn_name(group);

        constfn_declaration.push_str(&format!(
            "{EVAL_FEAT} pub const fn {fn_name}(ctx: &Ctx) -> f32 {{
                {body}
            }}\n",
            body = key.body
        ));

        for e in group {
            fn_name_by_slot.insert((e.tag, e.kind), fn_name.clone());
        }
    }

    let resolve = |tag, kind| {
        fn_name_by_slot
            .get(&(tag, kind))
            .cloned()
            .unwrap_or_else(|| "zero".into())
    };

    let mk_closure = |expr: &str| {
        let ctx = expr.ctx_param();
        let body = expr.to_lowercase();
        format!("|{ctx}| {body}")
    };

    let melee_closures = [
        mk_closure(&melee.minimum_damage),
        mk_closure(&melee.maximum_damage),
    ];

    let ranged_closures = [
        mk_closure(&ranged.minimum_damage),
        mk_closure(&ranged.maximum_damage),
    ];

    let melee_fn_names = vec![resolve("melee", "min"), resolve("melee", "max")];
    let ranged_fn_names = vec![resolve("ranged", "min"), resolve("ranged", "max")];

    let get_fn_call = |names: &[String]| {
        names
            .iter()
            .map(|f| format!("{f}(ctx)"))
            .collect::<Vec<_>>()
            .join(", ")
    };

    let match_arm = format!(
        "AttackType::Melee => [{}],
        AttackType::Ranged => [{}]",
        get_fn_call(&melee_fn_names),
        get_fn_call(&ranged_fn_names),
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
    html_closure: String,
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

        let html_declaration = [
            base_declaration.as_str(),
            &{
                let [melee_min, melee_max] = &melee_closures;
                let [ranged_min, ranged_max] = &ranged_closures;
                format!(
                    "melee_min_dmg: {melee_min},
                    melee_max_dmg: {melee_max},
                    ranged_min_dmg: {ranged_min},
                    ranged_max_dmg: {ranged_max},"
                )
            },
            &rest,
        ]
        .concat()
        .rust_fmt()
        .drop_f32s()
        .rust_html()
        .as_const();

        let html_closure = constfn_declaration
            .trim_start_matches(EVAL_FEAT)
            .trim_start_matches("pub const")
            .trim()
            .rust_fmt()
            .drop_f32s()
            .rust_html();

        let base_declaration = format!(
            "{EVAL_FEAT}{base_declaration}
                ranged_damages: [{ranged_fn_names}],
                melee_damages: [{melee_fn_names}], {rest}
                {constfn_declaration}"
        );

        let generator = CwdPath::get_generator(SrcFolder::Items, name_ssnake.to_lowercase())?;

        let idents = constfn_declaration
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
            html_closure,
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
        mut item_closures,
    ] = std::array::from_fn(|i| {
        let (name, vtype, feature) = [
            ("ITEM_CACHE", "&CachedItem", EVAL_FEAT),
            ("ITEM_ID_TO_NAME", "&str", GLOB_FEAT),
            ("ITEM_FORMULAS", "Range<usize>", GLOB_FEAT),
            ("ITEM_GENERATOR", "Range<usize>", GLOB_FEAT),
            ("ITEM_ID_TO_RIOT_ID", "u32", GLOB_FEAT),
            ("ITEM_IDENTS", "&[EvalIdent]", GLOB_FEAT),
            ("ITEM_CLOSURES", "Range<usize>", GLOB_FEAT),
        ][i];
        format!("{feature} pub static {name}: [{vtype}; ItemId::VARIANTS] = [")
    });

    let mut block = String::new();

    let mut tracker = Tracker::new(&mut block);
    let mut formula_offsets = Vec::with_capacity(len);
    let mut closure_offsets = Vec::with_capacity(len);
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
            html_closure,
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
        tracker.record_into(&html_closure, &mut closure_offsets);
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
            ctx: &Ctx, 
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
                target.push_str(&format!("({new_start}..{new_end}),"));
            }
            push_end([target], "];");
        };

        [
            (generator_offsets, &mut item_generator),
            (formula_offsets, &mut item_formulas),
            (closure_offsets, &mut item_closures),
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
            item_closures,
            const_eval,
        ]
        .concat()
        .rust_fmt();

        let exports = format!("pub mod items {{ use super::*; {content} }}");

        Generated { exports, block }
    };

    Ok(Box::new(callback))
}
