use {
    crate::{
        DynError, MayFail,
        generators::{
            constants::{Constants, WithConstants},
            parser::{DamageRange, GeneratorFnSig, Parser, WithMapChanges, model::RiotCdnRune},
            runes::rune_gen_fn,
        },
    },
    heck::ToPascalCase,
    serde::{Deserialize, Serialize},
    std::{
        collections::BTreeMap,
        fmt::Write,
        ops::{Index, IndexMut},
    },
    types::{AttackType, DamageIndex, DamageType, Key, MapMode, Tag},
    wiki::{
        JsonRead,
        map_changes::ChangeTarget,
        runes::{RuneKeystone, RuneSlot, WikiRune},
        wikitext::{
            Formula, IntoFormula, Lookup, Node, Nodes, PageContext, Scaling, SourceDocument,
            SourceForm, SourceRole, SourceVariant,
        },
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

        write!(
            result,
            "/* native source selection required for {} */",
            data.rune_id
        )?;

        Ok(true)
    }

    fn from_root(root: &std::path::Path) -> MayFail<Self> {
        super::load_map_changes(root)?;

        let mut data =
            BTreeMap::<String, WikiRune>::from_file(root.join("cache/wiki/runes/full.json"))?;

        let cdn = Vec::<RiotCdnRune>::from_file(root.join("cache/riot/runes.json"))?;

        for rune in data.values_mut() {
            rune.raw_source = std::fs::read_to_string(
                root.join("cache/wiki/runes")
                    .join(&rune.rune_id)
                    .join("data.lua"),
            )?;

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
                    raw_source: String::new(),
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

    /// Select a unique damage event from a named raw description field.
    pub fn native(&self, section: &str, attack: AttackType, form: SourceForm) -> MayFail<Formula> {
        let lookup = Lookup::new("return {}", "return {}")?;
        let context = PageContext {
            lookup: &lookup,
            four_rank_ultimate: false,
            ability_keys: BTreeMap::new(),
            crit_damage: 200.0,
        };

        let documents = SourceDocument::rune_descriptions(
            &self.data.raw_source,
            &context,
            SourceVariant {
                ranged: attack == AttackType::Ranged,
                alternate: false,
            },
        )?;

        let document = documents
            .get(section)
            .ok_or_else(|| format!("{} missing {section}", self.data.rune_id))?;

        let value = document
            .select(SourceRole::Damage, form)
            .map_err(|error| format!("{} / {section}: {error}", self.data.rune_id))?;

        self.mapped_formula(&self.data.name, attack, value.formula()?.clone())
    }

    /// Bonus attack speed explicitly granted for each stack, including map notes.
    pub fn attack_speed_per_stack(&self, attack: AttackType) -> MayFail<Formula> {
        let lookup = Lookup::new("return {}", "return {}")?;
        let context = PageContext {
            lookup: &lookup,
            four_rank_ultimate: false,
            ability_keys: BTreeMap::new(),
            crit_damage: 200.0,
        };

        let variant = SourceVariant {
            ranged: attack == AttackType::Ranged,
            alternate: false,
        };

        let nodes = Nodes::parse(&self.data.raw_source);
        let template = nodes
            .templates()
            .into_iter()
            .find(|t| t.named("description").is_some())
            .ok_or("Missing rune data")?;

        let description = template
            .named("description")
            .ok_or("Missing description")?
            .for_variant(variant)?;

        let mut selected = Vec::new();

        for pair in description.0.windows(2) {
            if let (Node::Template(template), Node::Text(after)) = (&pair[0], &pair[1])
                && template.name == "as"
                && after.trim_start().starts_with("for each stack")
            {
                let scaling = Scaling::new(template, Key::P).ok_or("Unreadable per-stack stat")?;

                if scaling.stat != "bonus attack speed" || scaling.ratios.len() != 1 {
                    return Err("Expected one per-stack bonus attack speed".into());
                }

                selected.push(Formula::Const(scaling.ratios[0]));
            }
        }

        let [mut formula] =
            <[_; 1]>::try_from(selected).map_err(|_| "Expected unique per-stack attack speed")?;

        for changes in super::map_changes() {
            for entry in changes
                .entries
                .values()
                .filter(|e| self.is_map_target(&e.target))
            {
                for change in entry.changes.iter().flat_map(|c| {
                    std::iter::once(c.text.as_str()).chain(c.notes.iter().map(String::as_str))
                }) {
                    if !change
                        .to_ascii_lowercase()
                        .contains("bonus attack speed per stack changed to")
                    {
                        continue;
                    }

                    let (_, value) = change
                        .split_once("changed to")
                        .ok_or("Missing per-stack map replacement")?;

                    let document = SourceDocument::parse(
                        "bonus attack speed per stack",
                        &format!("Gain {value}"),
                        &context,
                        variant,
                    )?;

                    let value = document.select(SourceRole::Stat, SourceForm::Base)?;

                    formula = formula.with_map(
                        super::single_mode(&changes.maps, "bonus attack speed per stack")?,
                        value.formula()?.clone(),
                    );
                }
            }
        }

        Ok(formula)
    }

    pub fn description(&self, section: &str) -> MayFail<String> {
        let nodes = Nodes::parse(&self.data.raw_source);
        let template = nodes
            .templates()
            .into_iter()
            .find(|t| t.named("description").is_some())
            .ok_or("Missing raw Rune data template")?;

        Ok(template
            .named(section)
            .ok_or("Missing named rune description")?
            .plain())
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

    pub fn assign_min(&mut self, damage: impl IntoFormula) -> &mut Self {
        self.assign(AttackType::Melee, DamageIndex::Min, &damage)
            .assign(AttackType::Ranged, DamageIndex::Min, damage)
    }

    pub fn assign_max(&mut self, damage: impl IntoFormula) -> &mut Self {
        self.assign(AttackType::Melee, DamageIndex::Max, &damage)
            .assign(AttackType::Ranged, DamageIndex::Max, damage)
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
