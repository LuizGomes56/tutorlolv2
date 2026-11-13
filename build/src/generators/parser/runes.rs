use {
    crate::{
        DynError, MayFail,
        generators::{
            constants::{Constants, WithConstants},
            impls::runes::rune_gen_fn,
            parser::{
                DamageRange, GeneratorFnSig, Parser, WithMapChanges, infer_damage_type,
                likely_damages, model::RiotCdnRune,
            },
        },
    },
    heck::ToPascalCase,
    serde::{Deserialize, Serialize},
    std::{
        collections::BTreeMap,
        fmt::Write,
        ops::{Index, IndexMut},
    },
    tutorlolv2_types::{AttackType, CtxVar, DamageIndex, DamageType, MapMode, Tag},
    tutorlolv2_wiki::{
        JsonRead,
        map_changes::{ChangeTarget, overrides::EffectOverrides},
        parser::Effect,
        runes::{RuneKeystone, RuneSlot, WikiRune},
        wikitext::{Formula, IntoFormula},
    },
};

pub struct RuneParser {
    pub data: BTreeMap<String, WikiRune>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Rune {
    pub riot_id: u32,
    pub data: WikiRune,
    pub damage_type: DamageType,
    pub ranged: DamageRange,
    pub melee: DamageRange,
    #[serde(default, skip_serializing_if = "Constants::is_empty")]
    pub constants: Constants,
    /// Formulas on each mode that changes them, for melee minimum and maximum,
    /// then ranged. Serialized, since the build reads the generator's output
    #[serde(default, skip_serializing_if = "super::no_map_damages")]
    pub map_damages: [Vec<(MapMode, String)>; 4],
}

impl Parser<WikiRune, Rune> for RuneParser {
    const TAG: Tag = Tag::Runes;
    const FN: fn(&str) -> Option<GeneratorFnSig<WikiRune, Rune>> = rune_gen_fn;

    fn map(&self) -> &BTreeMap<String, WikiRune> {
        &self.data
    }

    fn create_methods(&self, result: &mut String, id: &str) -> MayFail<bool> {
        let data = &self.data[id];

        for description in &data.descriptions {
            match likely_damages(description) {
                true => infer_damage_type(result, description),
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

    fn from_root(root: &std::path::Path) -> MayFail<Self> {
        super::load_map_changes(root)?;

        let mut data =
            BTreeMap::<String, WikiRune>::from_file(root.join("cache/wiki/runes/full.json"))?;

        let cdn = Vec::<RiotCdnRune>::from_file(root.join("cache/riot/runes.json"))?;

        for rune in data.values_mut() {
            let matches = |name: &str| name.eq_ignore_ascii_case(&rune.name);

            if let Some(id) = cdn.iter().find_map(|tree| {
                matches(&tree.name).then_some(tree.id).or_else(|| {
                    tree.slots.iter().find_map(|slot| {
                        slot.runes
                            .iter()
                            .find_map(|rune| matches(&rune.name).then_some(rune.id))
                    })
                })
            }) {
                rune.riot_id = id as _;
            }
        }

        let mut customize = |name: &str, riot_id| {
            let rune_id = name.to_pascal_case();

            data.insert(
                rune_id.clone(),
                WikiRune {
                    name: format!("{name} Shard"),
                    rune_id,
                    effects: Default::default(),
                    descriptions: Default::default(),
                    riot_id,
                    custom: true,
                    path: RuneKeystone::Inspiration,
                    slot: RuneSlot::Keystone,
                },
            );
        };

        // Stat shards are missing from `runesReforged.json`, but these are the
        // ids that riot uses for them in match data
        customize("Health", 5011);
        customize("Health Scaling", 5001);
        customize("Adaptive Force", 5008);
        customize("Attack Speed", 5005);
        customize("Ability Haste", 5007);
        customize("Move Speed", 5010);
        customize("Tenacity and Slow Resist", 5013);

        Ok(Self { data })
    }
}

impl WithMapChanges for Rune {
    fn is_map_target(&self, target: &ChangeTarget) -> bool {
        matches!(target, ChangeTarget::Rune { rune_id } if *rune_id == self.data.rune_id)
    }
}

impl TryFrom<WikiRune> for Rune {
    type Error = DynError;

    fn try_from(data: WikiRune) -> Result<Self, Self::Error> {
        let riot_id = data.riot_id as _;

        Ok(Self {
            riot_id,
            damage_type: Default::default(),
            ranged: Default::default(),
            melee: Default::default(),
            constants: Default::default(),
            map_damages: Default::default(),
            data,
        })
    }
}

#[allow(dead_code)]
impl Rune {
    pub fn damage_type(&mut self, damage_type: DamageType) -> &mut Self {
        self.damage_type = damage_type;
        self
    }

    pub fn formula(&self, index: usize) -> MayFail<Formula> {
        self.formula_for(index, AttackType::Melee)
    }

    pub fn formula_for(&self, index: usize, attack_type: AttackType) -> MayFail<Formula> {
        self.compose_effects(&[index], attack_type, |effect| {
            effect
                .to_formula(CtxVar::Level)?
                .ok_or_else(|| "No formula found for effect".into())
        })
    }

    /// Attach mode overrides before custom generators compose or scale the values.
    fn compose_effects(
        &self,
        indexes: &[usize],
        attack_type: AttackType,
        read: impl Fn(&Effect) -> MayFail<Formula>,
    ) -> MayFail<Formula> {
        let effects = indexes
            .iter()
            .map(|&index| {
                let (name, effect) = self
                    .data
                    .effects
                    .iter()
                    .nth(index)
                    .ok_or_else(|| format!("No effect found at index {index}"))?;

                Ok((name.clone(), (name.clone(), effect.clone())))
            })
            .collect::<MayFail<BTreeMap<_, _>>>()?;

        let sum = |overrides: &BTreeMap<String, Effect>| {
            indexes
                .iter()
                .map(|&index| {
                    let (name, effect) = self.data.effects.iter().nth(index).unwrap();
                    read(overrides.get(name).unwrap_or(effect))
                })
                .collect::<MayFail<Vec<_>>>()
                .map(Formula::Sum)
        };

        let mut formula = sum(&BTreeMap::new())?;
        let target = ChangeTarget::Rune {
            rune_id: self.data.rune_id.clone(),
        };

        for changes in super::map_changes() {
            if !changes.entries.values().any(|entry| entry.target == target) {
                continue;
            }

            let mode = super::single_mode(&changes.maps, &self.data.rune_id)?;
            let overrides = EffectOverrides::new(
                &target,
                changes,
                &effects,
                attack_type == AttackType::Ranged,
            );

            if !overrides.effects.is_empty() {
                formula = formula.with_map(mode, sum(&overrides.effects)?);
            }
        }

        Ok(formula)
    }

    pub fn effect(&self, index: usize) -> MayFail<&Effect> {
        self.data
            .effects
            .values()
            .nth(index)
            .ok_or_else(|| format!("No effect found at index {index}").into())
    }

    pub fn use_formula(&self, index: usize) -> MayFail<Formula> {
        self.use_formula_for(index, AttackType::Melee)
    }

    pub fn use_formula_for(&self, index: usize, attack_type: AttackType) -> MayFail<Formula> {
        self.compose_effects(&[index], attack_type, |effect| {
            effect.use_formula_tree(CtxVar::Level)
        })
    }

    pub fn description(&self, index: usize) -> MayFail<&String> {
        Ok(self
            .data
            .descriptions
            .get(index)
            .ok_or_else(|| format!("No description found at index {index}"))?)
    }

    pub fn scaling(&self, n: usize, indexes: impl Iterator<Item = usize>) -> MayFail<Formula> {
        self.scaling_for(n, indexes, AttackType::Melee)
    }

    pub fn scaling_for(
        &self,
        n: usize,
        indexes: impl Iterator<Item = usize>,
        attack_type: AttackType,
    ) -> MayFail<Formula> {
        let indexes = indexes.collect::<Vec<_>>();

        self.compose_effects(&[n], attack_type, |effect| {
            indexes
                .iter()
                .map(|&index| {
                    effect
                        .scalings
                        .get(index)
                        .ok_or_else(|| format!("No scaling found at index {index}").into())
                        .and_then(|scaling| scaling.to_formula(CtxVar::Level))
                })
                .collect::<MayFail<Vec<_>>>()
                .map(Formula::Sum)
        })
    }

    pub fn damage(
        &mut self,
        attack_type: AttackType,
        var: DamageIndex,
        index: usize,
    ) -> MayFail<&mut Self> {
        let formula = self.formula_for(index, attack_type)?;
        Ok(self.assign(attack_type, var, formula))
    }

    pub fn assign(
        &mut self,
        attack_type: AttackType,
        var: DamageIndex,
        damage: impl IntoFormula,
    ) -> &mut Self {
        let formula = damage.to_formula();
        let slot = match (attack_type, var) {
            (AttackType::Melee, DamageIndex::Min) => 0,
            (AttackType::Melee, DamageIndex::Max) => 1,
            (AttackType::Ranged, DamageIndex::Min) => 2,
            (AttackType::Ranged, DamageIndex::Max) => 3,
        };

        self.map_damages[slot] = formula
            .modes()
            .into_iter()
            .map(|mode| (mode, formula.on(mode).rust()))
            .collect();

        self[attack_type][var] = formula.on(MapMode::Default).rust();
        self
    }

    pub fn compose<const N: usize>(&mut self, indexes: [usize; N]) -> MayFail<Formula> {
        self.compose_for(indexes, AttackType::Melee)
    }

    pub fn compose_for<const N: usize>(
        &self,
        indexes: [usize; N],
        attack_type: AttackType,
    ) -> MayFail<Formula> {
        self.compose_effects(&indexes, attack_type, |effect| {
            effect
                .to_formula(CtxVar::Level)?
                .ok_or_else(|| "No formula found for effect".into())
        })
    }

    pub fn assign_min(&mut self, damage: impl IntoFormula) -> &mut Self {
        self.assign(AttackType::Melee, DamageIndex::Min, &damage)
            .assign(AttackType::Ranged, DamageIndex::Min, damage)
    }

    pub fn assign_max(&mut self, damage: impl IntoFormula) -> &mut Self {
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

impl WithConstants for Rune {
    fn constants_mut(&mut self) -> &mut Constants {
        &mut self.constants
    }
}
