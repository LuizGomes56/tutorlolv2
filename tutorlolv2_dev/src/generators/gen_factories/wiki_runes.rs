use crate::{
    GeneratorExt, JsonRead, MayFail,
    client::Tag,
    gen_factories::{DamageIndex, DamageRange, Parser, infer_damage_type, likely_damages},
    gen_runes::rune_gen_fn,
    gen_utils::RegExtractor,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    ops::{Index, IndexMut},
};
use tutorlolv2_types::{AttackType, CtxVar, DamageType};
use tutorlolv2_wiki::{parser::Effect, runes::WikiRune};

pub struct RuneParser {
    pub data: BTreeMap<String, WikiRune>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Rune {
    #[serde(flatten)]
    pub data: WikiRune,
    pub damage_type: DamageType,
    pub ranged: DamageRange,
    pub melee: DamageRange,
}

impl Parser<WikiRune, Rune> for RuneParser {
    const TAG: Tag = Tag::Runes;
    const FN: fn(&str) -> Option<fn(WikiRune) -> Box<dyn GeneratorExt<Rune>>> = rune_gen_fn;

    fn map(&self) -> &BTreeMap<String, WikiRune> {
        &self.data
    }

    fn create_methods(&self, result: &mut String, id: &str) -> bool {
        let data = &self.data[id];

        for description in &data.descriptions {
            match likely_damages(description) {
                true => infer_damage_type(result, &description),
                false => return false,
            }
        }

        for (i, (key, effect)) in data.effects.iter().enumerate() {
            if effect.formula.is_some() {
                result.push_str(&format!(".min({i})? /* {key} */"));
            }
        }

        true
    }

    fn new() -> MayFail<Self> {
        Ok(Self {
            data: BTreeMap::from_file("cache/wiki/runes/full.json")?,
        })
    }
}

impl From<WikiRune> for Rune {
    fn from(data: WikiRune) -> Self {
        Self {
            data,
            damage_type: Default::default(),
            ranged: Default::default(),
            melee: Default::default(),
        }
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
            .map(|v| v.replace("x", &format!("{level} as f32", level = CtxVar::Level)))
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

    pub fn end(&self) -> MayFail {
        if matches!(self.damage_type, DamageType::Unknown) {
            return Err("Unknown damage type for this rune".into());
        }

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
