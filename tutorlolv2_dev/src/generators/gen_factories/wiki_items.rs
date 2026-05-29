use crate::{
    DynError, GeneratorExt, JsonRead, MayFail,
    client::{SaveTo, Tag},
    gen_factories::{
        DamageRange, Parser, ZERO, get_identifiers, infer_damage_type, is_zero, likely_damages,
    },
    gen_items::item_gen_fn,
    gen_utils::RegExtractor,
    riot::{RiotCdn, RiotCdnItem, RiotCdnItemGold},
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    ops::{Index, IndexMut},
};
use tutorlolv2_types::{
    AttackType, CtxVar, DamageIndex, DamageType, GameMap, StatName, TypeMetadata,
};
use tutorlolv2_wiki::items::item_parser::{ItemEffect, WikiItem};

pub struct ItemParser {
    pub data: BTreeMap<String, WikiItem>,
}

impl Parser<WikiItem, Item> for ItemParser {
    const TAG: Tag = Tag::Items;
    const FN: fn(&str) -> Option<fn(WikiItem) -> MayFail<Box<dyn GeneratorExt<Item>>>> =
        item_gen_fn;

    fn map(&self) -> &BTreeMap<String, WikiItem> {
        &self.data
    }

    fn create_methods(&self, result: &mut String, id: &str) -> bool {
        let data = &self.data[id];

        match data.effects.act.as_ref().or(data.effects.pass.as_ref()) {
            Some(ie) => {
                let description = &ie.effect.inner.description;

                match likely_damages(description) {
                    true => infer_damage_type(result, description),
                    false => return false,
                }
            }
            None => return false,
        }

        let mut new_method = |field: &Option<ItemEffect>, tag| {
            if let Some(ie) = &field
                && ie.effect.formula.is_some()
            {
                result.push_str(&format!(".min({tag})?"));
            }
        };

        new_method(&data.effects.act, "Active");
        new_method(&data.effects.pass, "Passive");

        true
    }

    fn new() -> MayFail<Self> {
        let mut data = BTreeMap::<String, WikiItem>::from_file("cache/wiki/items/full.json")?;
        let riot_items = RiotCdn::<u32, RiotCdnItem>::from_file(SaveTo::RiotItems.path())?;

        for (id, cdn_item) in riot_items.data {
            let stats = cdn_item.pretiffy_stats().unwrap_or_default();

            let RiotCdnItem {
                name,
                gold: RiotCdnItemGold {
                    total, purchasable, ..
                },
                maps,
                from: recipe,
                ..
            } = cdn_item;

            let key = tutorlolv2_fmt::pascal_case(&name);

            match data.get_mut(&key) {
                Some(item) => item.purchasable = purchasable,
                None => {
                    if !name.is_empty() && !name.starts_with("<") {
                        let value = WikiItem {
                            id,
                            name,
                            item_id: key.clone(),
                            tier: Some(match total {
                                0..750 => 1,
                                750..2000 => 2,
                                _ => 3,
                            }),
                            modes: maps.into_iter().map(|(k, v)| (k.to_string(), v)).collect(),
                            stats: stats
                                .into_iter()
                                .map(|(k, v)| ((k as u8).to_string(), v as _))
                                .collect(),
                            effects: Default::default(),
                            recipe,
                            buy: Some(total as _),
                            purchasable,
                            custom: true,
                        };

                        data.insert(key, value);
                    }
                }
            }
        }

        Ok(Self { data })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Item {
    pub data: WikiItem,
    pub damage_type: DamageType,
    pub ranged: DamageRange,
    pub melee: DamageRange,
    pub build: ItemBuild,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ItemBuild {
    pub name: String,
    pub tier: u8,
    pub price: u16,
    pub stats: Vec<(StatName, u16)>,
    pub maps: Vec<GameMap>,
    pub metadata: TypeMetadata<String>,
    pub ranged: [String; 2],
    pub melee: [String; 2],
    pub deals_damage: [bool; 4],
    pub purchasable: bool,
    pub riot_id: u32,
    pub identifiers: [[BTreeSet<CtxVar>; 2]; 2],
    pub custom: bool,
}

#[derive(Copy, Clone, Debug)]
pub enum Source {
    Active,
    Passive,
}

impl TryFrom<WikiItem> for Item {
    type Error = DynError;

    fn try_from(data: WikiItem) -> Result<Self, Self::Error> {
        let damage = [ZERO.into(), ZERO.into()];

        Ok(Self {
            damage_type: Default::default(),
            ranged: Default::default(),
            melee: Default::default(),
            build: ItemBuild {
                name: data.name.clone(),
                tier: data.tier.unwrap_or(1),
                price: data.buy.unwrap_or(0),
                stats: data
                    .stats
                    .iter()
                    .filter_map(|(k, v)| {
                        let stat = k
                            .parse::<u8>()
                            .ok()
                            .map(StatName::from_u8)
                            .flatten()
                            .unwrap_or_else(|| match k.as_str() {
                                "ah" => StatName::AbilityHaste,
                                "hp" => StatName::Health,
                                "mr" => StatName::MagicResist,
                                "ap" => StatName::AbilityPower,
                                "mana" => StatName::Mana,
                                "ms" => StatName::MoveSpeedPercent,
                                "hsp" => StatName::HealAndShieldPower,
                                "mp5" => StatName::BaseManaRegen,
                                "armor" => StatName::Armor,
                                "msflat" => StatName::MoveSpeed,
                                "crit" => StatName::CritChance,
                                "ad" => StatName::AttackDamage,
                                "armpen" => StatName::ArmorPenetration,
                                "lethality" => StatName::Lethality,
                                "as" => StatName::AttackSpeed,
                                "lifesteal" => StatName::LifeSteal,
                                "mpen" => StatName::MagicPenetrationPercent,
                                "mpenflat" => StatName::MagicPenetration,
                                "gp10" => StatName::GoldPer10Seconds,
                                "hp5" => StatName::BaseHealthRegen,
                                "tenacity" => StatName::Tenacity,
                                "spec" => StatName::AdaptiveForce,
                                "omnivamp" => StatName::Omnivamp,
                                "hp5flat" => StatName::BaseHealthRegen,
                                "critdamage" => StatName::CritDamage,
                                _ => unreachable!(
                                    "Found unknown stat: {k} for {item_id}",
                                    item_id = data.item_id
                                ),
                            });

                        Some((stat, *v as _))
                    })
                    .collect(),
                maps: data
                    .modes
                    .iter()
                    .filter(|(_, v)| **v)
                    .filter_map(|(k, _)| {
                        k.parse::<u8>()
                            .ok()
                            .map(GameMap::from_u8)
                            .or_else(|| match k.as_str() {
                                "ar" => Some(GameMap::Arena),
                                "aram" => Some(GameMap::Aram),
                                "classic sr 5v5" => Some(GameMap::SummonersRift),
                                "nb" => Some(GameMap::NexusBlitz),
                                _ => None,
                            })
                    })
                    .collect(),
                metadata: TypeMetadata {
                    kind: data.item_id.clone(),
                    damage_type: Default::default(),
                    attributes: Default::default(),
                },
                ranged: damage.clone(),
                melee: damage,
                riot_id: data.id,
                deals_damage: Default::default(),
                purchasable: data.purchasable,
                identifiers: Default::default(),
                custom: data.custom,
            },
            data,
        })
    }
}

impl Item {
    pub fn damage_type(&mut self, damage_type: DamageType) -> &mut Self {
        self.damage_type = damage_type;
        self
    }

    pub fn formula(&self, source: Source) -> MayFail<String> {
        match &self[source] {
            Some(ie) if let Some(ref formula) = ie.effect.formula => Ok(formula.parenthesize()),
            _ => {
                println!(
                    "[{name}] No formula for its {source:?}",
                    name = self.data.name
                );
                Ok(ZERO.into())
            }
        }
    }

    pub fn assign(
        &mut self,
        attack_type: AttackType,
        var: DamageIndex,
        damage: impl AsRef<str>,
    ) -> &mut Self {
        self[attack_type][var] = damage.as_ref().to_string();
        self
    }

    pub fn damage(
        &mut self,
        attack_type: AttackType,
        var: DamageIndex,
        index: Source,
    ) -> MayFail<&mut Self> {
        let formula = self.formula(index)?;
        Ok(self.assign(attack_type, var, formula))
    }

    pub fn asgn_min(&mut self, damage: impl AsRef<str>) -> &mut Self {
        self.assign(AttackType::Melee, DamageIndex::Min, &damage)
            .assign(AttackType::Ranged, DamageIndex::Min, damage)
    }

    pub fn asgn_max(&mut self, damage: impl AsRef<str>) -> &mut Self {
        self.assign(AttackType::Melee, DamageIndex::Max, &damage)
            .assign(AttackType::Ranged, DamageIndex::Max, damage)
    }

    pub fn min(&mut self, index: Source) -> MayFail<&mut Self> {
        self.damage(AttackType::Melee, DamageIndex::Min, index)?;
        self.damage(AttackType::Ranged, DamageIndex::Min, index)
    }

    pub fn max(&mut self, index: Source) -> MayFail<&mut Self> {
        self.damage(AttackType::Melee, DamageIndex::Max, index)?;
        self.damage(AttackType::Ranged, DamageIndex::Max, index)
    }

    pub fn end(&mut self) -> MayFail {
        if matches!(self.damage_type, DamageType::Unspecified) {
            println!(
                "[warn] {item_id} has unknown damage type",
                item_id = self.data.item_id
            )
            // return Err("Unknown damage type for this item".into());
        }

        self.build.metadata.damage_type = self.damage_type;
        self.build.melee = [self.melee.min_dmg.clone(), self.melee.max_dmg.clone()];
        self.build.ranged = [self.ranged.min_dmg.clone(), self.ranged.max_dmg.clone()];
        self.build.deals_damage = [
            self.melee.min_dmg.as_str(),
            &self.melee.max_dmg,
            &self.ranged.min_dmg,
            &self.ranged.max_dmg,
        ]
        .map(|v| !is_zero(v));

        self.build.identifiers = core::array::from_fn(|i| {
            let attack_type = match i {
                0 => AttackType::Melee,
                1 => AttackType::Ranged,
                _ => unreachable!(),
            };

            core::array::from_fn(|j| {
                let damage_index = match j {
                    0 => DamageIndex::Min,
                    1 => DamageIndex::Max,
                    _ => unreachable!(),
                };

                get_identifiers(&self[attack_type][damage_index], self.damage_type)
                    .collect::<BTreeSet<_>>()
                    .into_iter()
                    .collect()
            })
        });

        Ok(())
    }
}

impl Index<AttackType> for Item {
    type Output = DamageRange;

    fn index(&self, index: AttackType) -> &Self::Output {
        match index {
            AttackType::Melee => &self.melee,
            AttackType::Ranged => &self.ranged,
        }
    }
}

impl IndexMut<AttackType> for Item {
    fn index_mut(&mut self, index: AttackType) -> &mut Self::Output {
        match index {
            AttackType::Melee => &mut self.melee,
            AttackType::Ranged => &mut self.ranged,
        }
    }
}

impl Index<Source> for Item {
    type Output = Option<ItemEffect>;

    fn index(&self, index: Source) -> &Self::Output {
        match index {
            Source::Active => &self.data.effects.act,
            Source::Passive => &self.data.effects.pass,
        }
    }
}

impl IndexMut<Source> for Item {
    fn index_mut(&mut self, index: Source) -> &mut Self::Output {
        match index {
            Source::Active => &mut self.data.effects.act,
            Source::Passive => &mut self.data.effects.pass,
        }
    }
}
