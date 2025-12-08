use crate::{
    _lib::{_parallel, CwdPath, Generated, MayFail},
    scripts::{DamageObject, Item, ItemStats, StringExt},
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

    let name = name.remove_special_chars();

    let metadata = format!(
        "TypeMetadata {{ 
            kind: ItemId::{name}, 
            damage_type: DamageType::{damage_type}, 
            attributes: Attrs::{attributes:?} 
        }}"
    );

    let assign_value = |expr: &str| {
        (!(expr.is_empty() || expr == "zero")).then_some({
            let (new_expr, changed) = expr.clean_math_expr().transform_expr();
            let ctx_param = if changed { "ctx" } else { "_" };
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

pub async fn generate_items() -> MayFail<impl FnOnce(usize) -> Generated> {
    struct ItemResult {
        riot_id: usize,
        item_formula: String,
        declaration: String,
        name: String,
        name_ssnake: String,
        name_normalized: String,
        generator: String,
    }

    let mut data = _parallel(32, "internal/items", async |item_id, item: Item| {
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

        let name_ssnake = name.to_screaming_snake_case();
        let name_normalized = name.remove_special_chars();
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

        let is_simulated = tier >= 3 && price > 0 && purchasable;
        let is_damaging = {
            let is_zeroed = |expr: &str| expr != "zero" && !expr.is_empty();
            is_zeroed(&ranged.minimum_damage)
                || is_zeroed(&ranged.maximum_damage)
                || is_zeroed(&melee.minimum_damage)
                || is_zeroed(&melee.maximum_damage)
        };

        let declaration = format!(
            "pub static {name}: CachedItem = CachedItem {{
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
            }};",
            name = format_args!("{name_ssnake}_{riot_id}"),
            stats = format_stats(&stats),
        );

        let generator_path = CwdPath::new(format!(
            "tutorlolv2_dev/src/generators/gen_items/{name}.rs",
            name = name_ssnake.to_lowercase()
        ));
        let generator = tokio::fs::read_to_string(generator_path)
            .await?
            .invoke_rustfmt(80)
            .highlight_rust();

        Ok(ItemResult {
            riot_id,
            item_formula: declaration
                .invoke_rustfmt(70)
                .clear_suffixes()
                .highlight_rust()
                .replace_const(),
            declaration,
            generator,
            name_ssnake,
            name_normalized,
            name: item.name,
        })
    })
    .await?
    .into_iter()
    .map(|(_, value)| value)
    .collect::<Vec<_>>();

    data.sort_by(|a, b| a.riot_id.cmp(&b.riot_id));

    Ok(|_| todo!())
}
