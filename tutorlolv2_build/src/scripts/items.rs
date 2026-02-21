use crate::{
    CwdPath, DEFAULT_ITEM_GENERATOR_OFFSET, Generated, GeneratorFn, SrcFolder, Tracker,
    ZERO_FN_OFFSET, parallel_task, push_end,
    scripts::{
        StringExt,
        model::{Item, ItemStats, MerakiItemStatMap},
        rustfmt_batch,
    },
};
use std::collections::{HashMap, HashSet};

struct DeclaredItem {
    metadata: String,
    ranged_closures: [String; 2],
    melee_closures: [String; 2],
    constfn_declaration: String,
    melee_fn_names: String,
    ranged_fn_names: String,
    match_arm: String,
}

const MAX_TUPLE_0: (usize, usize) = (usize::MAX, usize::MAX);
const MAX_TUPLE_1: (usize, usize) = (usize::MAX - 1, usize::MAX);

fn declare_item(name: &str, item: &Item) -> DeclaredItem {
    let Item {
        damage_type,
        ranged,
        melee,
        attributes,
        ..
    } = item;

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
            "pub const fn {fn_name}(ctx: &Ctx) -> f32 {{
                {body}
            }}",
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

    let melee_fn_names = [resolve("melee", "min"), resolve("melee", "max")];
    let ranged_fn_names = [resolve("ranged", "min"), resolve("ranged", "max")];

    let get_fn_call = |names: &[String]| {
        names
            .iter()
            .map(|f| format!("{f}(ctx)"))
            .collect::<Vec<_>>()
            .join(", ")
    };

    let melee_fn_calls = get_fn_call(&melee_fn_names);
    let ranged_fn_calls = get_fn_call(&ranged_fn_names);

    let match_arm = match melee_fn_calls == ranged_fn_calls {
        true => format!("[{melee_fn_calls}]"),
        false => format!(
            "match attack_type {{
                AttackType::Melee => [{melee_fn_calls}],
                AttackType::Ranged => [{ranged_fn_calls}]
            }}"
        ),
    };

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
                let value = stats.$field.flat;
                if value != 0.0 {
                    all_stats.push(
                        format!(
                            "{field}: {value}f32",
                            field = stringify!($field),
                        )
                    );
                }
            )+
            $(
                $(
                    let value = stats.$pfield.$disc;
                    if value != 0.0 {
                        all_stats.push(
                            format!(
                                "{field}: {value}f32",
                                field = concat!(
                                    stringify!($pfield),
                                    "_",
                                    stringify!($disc)
                                )
                            )
                        );
                    }
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

    match all_stats.len() {
        0 => "ZEROED_STATS".into(),
        len => {
            if len < NUMBER_OF_STATS {
                all_stats.push("..ZEROED_STATS".into());
            }
            let stats = all_stats.join(",");
            format!("CachedItemStats {{ {stats} }}")
        }
    }
}

struct ItemResult {
    riot_id: usize,
    html_declaration: String,
    base_declaration: String,
    name: String,
    name_ssnake: String,
    name_pascal: String,
    generator: Option<String>,
    match_arm: String,
    idents: String,
    html_closure: String,
    deals_damage: bool,
}

pub fn generate_items() -> GeneratorFn {
    let mut data = parallel_task("internal/items", |name_pascal, item: Item| {
        let DeclaredItem {
            metadata,
            match_arm,
            ranged_closures,
            melee_closures,
            melee_fn_names,
            ranged_fn_names,
            constfn_declaration,
        } = declare_item(name_pascal, &item);

        let Item {
            riot_id,
            name,
            prettified_stats,
            tier,
            price,
            purchasable,
            stats,
            ranged,
            melee,
            ..
        } = item;

        let name_ssnake = name.to_ssnake();
        let prettified_stats = prettified_stats
            .iter()
            .map(|(stat, value)| format!("(StatName::{stat:?}, {value})"))
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

        let maps = item
            .maps
            .into_iter()
            .map(|(key, val)| {
                let map_name = key.to_ssnake().to_lowercase();
                format!("{map_name}: {val},")
            })
            .collect::<String>();

        let rest = format!(
            "riot_id: {riot_id},
            deals_damage: {deals_damage},
            stats: {stats},
            maps: ItemMaps {{{maps}}}}};"
        );

        let base_declaration = format!(
            "pub static {name_ssnake}: CachedItem = CachedItem {{
                name: {name:?},
                price: {price},
                prettified_stats: &[{prettified_stats}],
                metadata: {metadata},
                tier: {tier},
                purchasable: {purchasable},"
        );

        let html_declaration = [
            base_declaration.as_str(),
            &{
                match deals_damage {
                    true => {
                        let [melee_min, melee_max] = &melee_closures;
                        let [ranged_min, ranged_max] = &ranged_closures;
                        format!(
                            "melee_min_dmg: {melee_min},
                            melee_max_dmg: {melee_max},
                            ranged_min_dmg: {ranged_min},
                            ranged_max_dmg: {ranged_max},"
                        )
                    }
                    false => "damage: zero,".into(),
                }
            },
            &rest,
        ]
        .concat();

        let html_closure = constfn_declaration
            .replace("pub const ", "")
            .trim()
            .to_string();

        let base_declaration = format!(
            "{base_declaration}
            ranged_damages: [{ranged_fn_names}],
            melee_damages: [{melee_fn_names}], {rest}
            {constfn_declaration}"
        );

        let generator =
            CwdPath::get_generator(SrcFolder::Items, name_pascal.to_ssnake().to_lowercase()).ok();

        let idents = constfn_declaration
            .get_idents(&item.damage_type)
            .into_iter()
            .collect::<String>();

        println!("[build] ItemId::{name_pascal}");

        Ok(ItemResult {
            riot_id,
            match_arm,
            html_declaration,
            base_declaration,
            generator,
            name_ssnake,
            name_pascal: name_pascal.into(),
            html_closure,
            name,
            deals_damage,
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
        mut item_formulas,
        mut item_generator,
        mut item_idents,
        mut item_closures,
    ] = std::array::from_fn(|i| {
        let (name, vtype) = [
            ("ITEM_CACHE", "&CachedItem"),
            ("ITEM_FORMULAS", "Range<usize>"),
            ("ITEM_GENERATOR", "Range<usize>"),
            ("ITEM_IDENTS", "&[CtxVar]"),
            ("ITEM_CLOSURES", "Range<usize>"),
        ][i];
        format!("pub static {name}: [{vtype}; ItemId::VARIANTS] = [")
    });

    let mut block = String::new();

    let mut tracker = Tracker::new(&mut block);

    let mut formula_offsets = Vec::with_capacity(len);
    let mut closure_offsets = Vec::with_capacity(len);
    let mut generator_offsets = Vec::with_capacity(len);

    let mut item_id_enum_match_arms = Vec::with_capacity(len);
    let mut item_id_enum_fields = Vec::with_capacity(len);
    let mut const_match_arms = String::new();
    let mut rustfmt_inputs = Vec::with_capacity(len * 2);

    for (
        _,
        ItemResult {
            riot_id,
            match_arm,
            deals_damage,
            html_declaration,
            base_declaration,
            name_ssnake,
            name_pascal,
            generator,
            html_closure,
            idents,
            ..
        },
    ) in data
    {
        if deals_damage {
            let match_arm = format!("ItemId::{name_pascal} => {{{match_arm}}}");
            const_match_arms.push_str(&match_arm);
        }
        item_idents.push_str(&format!("{idents},"));
        item_id_enum_match_arms.push(format!("{riot_id} => Some(Self::{name_pascal})"));
        item_id_enum_fields.push(name_pascal);
        item_cache.push_str(&format!("&{name_ssnake},"));
        item_declarations.push_str(&base_declaration);

        match generator {
            Some(generator) => tracker.record_into(&generator.rust_html(), &mut generator_offsets),
            None => generator_offsets.push(MAX_TUPLE_0),
        }
        rustfmt_inputs.push(html_declaration);
        rustfmt_inputs.push(html_closure);
    }

    let formatted = rustfmt_batch(&rustfmt_inputs);

    for i in 0..len {
        let decl_fmt = &formatted[i * 2];
        let clos_fmt = &formatted[i * 2 + 1];
        let html_declaration = decl_fmt.drop_f32s().rust_html().as_const();
        tracker.record_into(&html_declaration, &mut formula_offsets);
        match clos_fmt.trim().is_empty() {
            true => closure_offsets.push(MAX_TUPLE_1),
            false => tracker.record_into(&clos_fmt.drop_f32s().rust_html(), &mut closure_offsets),
        }
    }

    let fields = item_id_enum_fields.join(",");
    let match_arms = item_id_enum_match_arms.join(",");

    let item_id_enum = format!(
        r#"
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[derive(bincode::Encode, bincode::Decode)]
        #[derive(serde::Serialize, serde::Deserialize)]
        #[repr(u16)]
        pub enum ItemId {{ {fields} }}
        impl ItemId {{
            pub const VARIANTS: usize = {len};
            pub const fn from_riot_id(id: u32) -> Option<Self> {{
                match id {{ {match_arms}, _ => None }}
            }}
        }}"#
    );

    let const_eval = format!(
        "pub const fn item_const_eval(
            ctx: &Ctx, 
            item_id: ItemId, 
            attack_type: AttackType
        ) -> [f32; 2] {{
            match item_id {{ {const_match_arms}, _ => [0.0, 0.0] }}
        }}"
    );

    push_end([&mut item_cache, &mut item_idents], "];");

    println!("[ok] Finished building items");

    let callback = move |index: usize| {
        let add_offsets = |(list, target): (Vec<_>, &mut String)| {
            for tuple in list {
                match tuple {
                    MAX_TUPLE_0 => {
                        let (s, e) = unsafe { DEFAULT_ITEM_GENERATOR_OFFSET };
                        target.push_str(&format!("({s}..{e}),"));
                    }
                    MAX_TUPLE_1 => {
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

        [
            (generator_offsets, &mut item_generator),
            (formula_offsets, &mut item_formulas),
            (closure_offsets, &mut item_closures),
        ]
        .into_iter()
        .for_each(add_offsets);

        let content = [
            item_id_enum,
            item_formulas,
            item_generator,
            item_cache,
            item_declarations,
            item_idents,
            item_closures,
            const_eval,
        ]
        .concat();

        let exports = format!("pub mod items {{ use super::*; {content} }}");

        Generated { exports, block }
    };

    Ok(Box::new(callback))
}
