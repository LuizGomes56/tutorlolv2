//! Generates [`Monster`] and the stats of each one from the parsed monster pages.

use crate::MayFail;
use heck::ToUpperCamelCase;
use std::fmt::Write;
use wiki::monsters::{MonsterStat, Monsters};

pub fn monsters_code(monsters: &Monsters) -> MayFail<String> {
    if monsters.is_empty() {
        return Err("[monsters] No monster was parsed".into());
    }

    let mut variants = String::new();
    let mut data = String::new();

    for (name, monster) in monsters {
        let variant = name.to_upper_camel_case();

        writeln!(variants, "/// {name}\n{variant},")?;
        writeln!(
            data,
            "MonsterStats {{ health: {}, armor: {}, magic_resist: {} }},",
            stat(&monster.health)?,
            stat(&monster.armor)?,
            stat(&monster.magic_resist)?,
        )?;
    }

    Ok(format!(
        "pub mod monsters_code {{
            use crate::model::{{MonsterStat, MonsterStats}};

            /// Every monster and minion read from the wiki, sorted by name
            #[derive(
                Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash,
                strum::EnumCount, strum::VariantArray, strum::VariantNames,
                strum::IntoStaticStr, strum::FromRepr,
            )]
            #[repr(u8)]
            pub enum Monster {{ {variants} }}

            impl Monster {{
                pub const DATA: [MonsterStats; <Self as strum::EnumCount>::COUNT] = [{data}];
            }}
        }}"
    ))
}

fn stat(stat: &MonsterStat) -> MayFail<String> {
    let float = |value: f64| format!("{:?}f32", value as f32);

    Ok(match stat {
        MonsterStat::Flat(value) => format!("MonsterStat::Flat({})", float(*value)),
        MonsterStat::Levels { values, .. } if values.is_empty() => {
            return Err("[monsters] A level table has no values".into());
        }
        MonsterStat::Levels { start, values } => format!(
            "MonsterStat::Levels {{ start: {start}, values: &[{}] }}",
            values
                .iter()
                .map(|&value| float(value))
                .collect::<Vec<_>>()
                .join(",")
        ),
        MonsterStat::Upgrades {
            base,
            per_upgrade,
            cap,
        } => format!(
            "MonsterStat::Upgrades {{ base: {}, per_upgrade: {}, cap: {} }}",
            float(*base),
            float(*per_upgrade),
            float(*cap)
        ),
        MonsterStat::TriangularUpgrades {
            coefficient,
            after,
            cap,
        } => format!(
            "MonsterStat::TriangularUpgrades {{ coefficient: {}, after: {after}, cap: {} }}",
            float(*coefficient),
            float(*cap)
        ),
    })
}
