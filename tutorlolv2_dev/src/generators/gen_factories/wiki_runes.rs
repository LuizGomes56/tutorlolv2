use crate::{
    DynError, GeneratorExt, JsonRead, MayFail,
    client::{SaveTo, Tag},
    gen_factories::{
        DamageRange, Parser, ZERO, get_identifiers, infer_damage_type, is_zero, likely_damages,
    },
    gen_runes::rune_gen_fn,
    gen_utils::RegExtractor,
    riot::RiotCdnRune,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Write,
    ops::{Index, IndexMut},
};
use tutorlolv2_types::{AttackType, CtxVar, DamageIndex, DamageType, TypeMetadata};
use tutorlolv2_wiki::{
    parser::Effect,
    runes::{RuneKeystone, RuneSlot, WikiRune},
};

pub struct RuneParser {
    pub data: BTreeMap<String, WikiRune>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Rune {
    pub data: WikiRune,
    pub damage_type: DamageType,
    pub ranged: DamageRange,
    pub melee: DamageRange,
    pub build: RuneBuild,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RuneBuild {
    pub name: String,
    pub metadata: TypeMetadata<String>,
    pub melee: [String; 2],
    pub ranged: [String; 2],
    pub riot_id: u32,
    pub deals_damage: [bool; 4],
    pub identifiers: [[BTreeSet<CtxVar>; 2]; 2],
    pub custom: bool,
}

impl Parser<WikiRune, Rune> for RuneParser {
    const TAG: Tag = Tag::Runes;
    const FN: fn(&str) -> Option<fn(WikiRune) -> MayFail<Box<dyn GeneratorExt<Rune>>>> =
        rune_gen_fn;

    fn map(&self) -> &BTreeMap<String, WikiRune> {
        &self.data
    }

    fn create_methods(&self, result: &mut String, id: &str) -> MayFail<bool> {
        let data = &self.data[id];

        for description in &data.descriptions {
            match likely_damages(description) {
                true => infer_damage_type(result, &description),
                false => return Ok(false),
            }
        }

        for (i, (key, effect)) in data.effects.iter().enumerate() {
            if effect.formula.is_some() {
                write!(result, ".min({i})? /* {key} */")?;
            }
        }

        Ok(true)
    }

    fn new() -> MayFail<Self> {
        let mut data = BTreeMap::from_file("cache/wiki/runes/full.json")?;

        let mut customize = |name, riot_id| {
            let rune_id = tutorlolv2_fmt::pascal_case(name);
            data.insert(
                rune_id.clone(),
                WikiRune {
                    name: format!("{name} Shard"),
                    rune_id,
                    path: RuneKeystone::Domination,
                    slot: RuneSlot::Keystone,
                    effects: Default::default(),
                    descriptions: Default::default(),
                    riot_id,
                    custom: true,
                },
            );
        };

        customize("Health", 8980);
        customize("Health Scaling", 8981);
        customize("Adaptive Force", 8982);
        customize("Attack Speed", 8983);

        Ok(Self { data })
    }
}

impl TryFrom<WikiRune> for Rune {
    type Error = DynError;

    fn try_from(data: WikiRune) -> Result<Self, Self::Error> {
        let damage = [ZERO.into(), ZERO.into()];
        let name = data.name.clone();

        let riot_id = Vec::<RiotCdnRune>::from_file(SaveTo::RiotRunes.path())
            .ok()
            .and_then(|runes| {
                runes.into_iter().find_map(|cdn_rune| {
                    (cdn_rune.name == name).then_some(cdn_rune.id).or_else(|| {
                        cdn_rune.slots.into_iter().find_map(|slot| {
                            slot.runes
                                .into_iter()
                                .find_map(|tree| (tree.name == name).then_some(tree.id))
                        })
                    })
                })
            })
            .unwrap_or(data.riot_id) as _;

        Ok(Self {
            damage_type: Default::default(),
            ranged: Default::default(),
            melee: Default::default(),
            build: RuneBuild {
                name,
                metadata: TypeMetadata {
                    kind: data.rune_id.clone(),
                    damage_type: Default::default(),
                    attributes: Default::default(),
                },
                melee: damage.clone(),
                ranged: damage,
                riot_id,
                deals_damage: Default::default(),
                identifiers: Default::default(),
                custom: data.custom,
            },
            data,
        })
    }
}

impl Rune {
    pub fn damage_type(&mut self, damage_type: DamageType) -> &mut Self {
        self.damage_type = damage_type;
        self
    }

    pub fn formula(&self, index: usize) -> MayFail<String> {
        self.effect(index).and_then(|v| {
            v.formula
                .as_ref()
                .ok_or_else(|| format!("No formula string found for effect {index}").into())
                .map(RegExtractor::parenthesize)
        })
    }

    pub fn effect(&self, index: usize) -> MayFail<&Effect> {
        self.data
            .effects
            .values()
            .nth(index)
            .ok_or_else(|| format!("No effect found at index {index}").into())
    }

    pub fn use_formula(&self, index: usize) -> MayFail<String> {
        self.effect(index)?
            .use_formula
            .as_deref()
            .map(|v| v.replace("x", &format!("{level}", level = CtxVar::Level)))
            .ok_or_else(|| format!("No use formula found at index {index}").into())
    }

    pub fn description(&self, index: usize) -> MayFail<&String> {
        Ok(self
            .data
            .descriptions
            .get(index)
            .ok_or_else(|| format!("No description found at index {index}"))?)
    }

    pub fn scaling(&self, n: usize, indexes: impl Iterator<Item = usize>) -> MayFail<String> {
        let filter = indexes.collect::<Vec<_>>();
        Ok(self
            .effect(n)?
            .scalings
            .iter()
            .enumerate()
            .filter(|(i, _)| filter.contains(i))
            .filter_map(|(_, scaling)| scaling.render(CtxVar::Level).ok())
            .collect::<Vec<_>>()
            .join(" + "))
    }

    pub fn damage(
        &mut self,
        attack_type: AttackType,
        var: DamageIndex,
        index: usize,
    ) -> MayFail<&mut Self> {
        let formula = self.formula(index)?;
        Ok(self.assign(attack_type, var, formula))
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

    pub fn compose<const N: usize>(&mut self, indexes: [usize; N]) -> MayFail<String> {
        let mut result = [const { String::new() }; N];
        for (i, index) in indexes.into_iter().enumerate() {
            let formula = self.formula(index)?;
            result[i] = formula;
        }
        Ok(result.join(" + "))
    }

    pub fn assign_min(&mut self, damage: impl AsRef<str>) -> &mut Self {
        self.assign(AttackType::Melee, DamageIndex::Min, &damage)
            .assign(AttackType::Ranged, DamageIndex::Min, damage)
    }

    pub fn assign_max(&mut self, damage: impl AsRef<str>) -> &mut Self {
        self.assign(AttackType::Melee, DamageIndex::Max, &damage)
            .assign(AttackType::Ranged, DamageIndex::Max, damage)
    }

    pub fn min(&mut self, index: usize) -> MayFail<&mut Self> {
        self.damage(AttackType::Melee, DamageIndex::Min, index)?;
        self.damage(AttackType::Ranged, DamageIndex::Min, index)
    }

    pub fn max(&mut self, index: usize) -> MayFail<&mut Self> {
        self.damage(AttackType::Melee, DamageIndex::Max, index)?;
        self.damage(AttackType::Ranged, DamageIndex::Max, index)
    }

    pub fn end(&mut self) -> MayFail {
        if matches!(self.damage_type, DamageType::Unspecified) {
            println!(
                "[warn] {rune_id} has unknown damage type",
                rune_id = self.data.rune_id
            );
            // return Err("Unknown damage type for this rune".into());
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

impl Index<AttackType> for Rune {
    type Output = DamageRange;

    fn index(&self, index: AttackType) -> &Self::Output {
        match index {
            AttackType::Melee => &self.melee,
            AttackType::Ranged => &self.ranged,
        }
    }
}

impl IndexMut<AttackType> for Rune {
    fn index_mut(&mut self, index: AttackType) -> &mut Self::Output {
        match index {
            AttackType::Melee => &mut self.melee,
            AttackType::Ranged => &mut self.ranged,
        }
    }
}
