//! Champion generators v2: they select effects of the wikitext model
//! (`cache/wiki/champions/model.json`) by label and edit their [`Formula`] trees, from
//! which both the generated code and the codec's source are written. A champion uses
//! its v2 generator once it is ported, and its v1 generator until then
//!
//! ```ignore
//! impl GeneratorV2 for Darius {
//!     fn generate(&mut self) -> MayFail {
//!         self.take_nth("Hemorrhage", 0, P(Min))?
//!             .derive(P(Max), P(Min), |p| f![5 * p])?
//!             .take("True Damage", R(Min))?
//!             .combos([&[Attack, Ability(R(Min))]])?
//!             .end()
//!     }
//! }
//! ```
//!
//! Calls that can fail return `MayFail<&mut Self>`, and the others `&mut Self`

pub mod formula;
pub mod impls;
pub mod imports;

use {
    crate::{
        MayFail,
        generators::{
            constants::{Constant, WithConstants},
            parser::champions::{Ability, Champion},
        },
    },
    std::{collections::BTreeMap, path::Path, sync::LazyLock},
    tutorlolv2_types::{AbilityId, Attrs, ComboElement, DamageType, Key, MapMode},
    tutorlolv2_wiki::{
        JsonRead,
        wikitext::{AbilityModel, ChampionModel, EffectModel, Formula},
    },
};

/// A v2 champion generator, written on `self` as v1's
pub trait GeneratorV2 {
    fn generate(&mut self) -> MayFail;
}

/// How the formulas that game modes change are written in the generated code
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MapStyle {
    /// A branch on each value that a mode changes, inside the formula
    #[default]
    Value,
    /// One whole formula per mode, as v1 writes them
    Whole,
}

/// A damage of a champion, as it is taken from an effect and edited
#[derive(Clone, Debug)]
pub struct Damage {
    pub formula: Formula,
    /// Name of the ability
    pub name: String,
    pub damage_type: DamageType,
    /// The effect's label, unless a generator sets another one
    pub comment: String,
    pub attributes: Attrs,
}

/// What a v2 generator edits: the damages it takes from the model, and v1's
/// [`Champion`] for combos, constants, exceptions and the checks of its `end`
pub struct ChampionV2 {
    pub inner: Champion,
    pub model: ChampionModel,
    damages: BTreeMap<AbilityId, Damage>,
    style: MapStyle,
}

impl ChampionV2 {
    pub fn new(inner: Champion, model: ChampionModel) -> Self {
        Self {
            inner,
            model,
            damages: BTreeMap::new(),
            style: MapStyle::default(),
        }
    }

    /// Models of every champion in `path`, as `cache/wiki/champions/model.json`
    pub fn load_models(path: &Path) -> MayFail<BTreeMap<String, ChampionModel>> {
        BTreeMap::from_file(path).map_err(|e| {
            format!(
                "Can't read the champion models at {path:?}: {e}. Run `cargo run -- wiki cpm` in tutorlolv2_wiki"
            )
            .into()
        })
    }

    /// Models of every champion, read once from `cache/wiki/champions/model.json`
    pub fn models() -> MayFail<&'static BTreeMap<String, ChampionModel>> {
        static MODELS: LazyLock<Result<BTreeMap<String, ChampionModel>, String>> =
            LazyLock::new(|| {
                let path = crate::project_root().join("cache/wiki/champions/model.json");
                ChampionV2::load_models(&path).map_err(|e| e.to_string())
            });

        MODELS.as_ref().map_err(|e| e.clone().into())
    }

    fn champion_id(&self) -> &str {
        &self.inner.champion_id
    }

    /// Every effect of `key`, in page and tab order, then the values of the pages'
    /// descriptions, labeled by their heading as Darius's `"Hemorrhage"`
    fn effects(&self, key: Key) -> Vec<(&AbilityModel, &EffectModel)> {
        let pages = || self.model.abilities.get(&key).into_iter().flatten();

        pages()
            .flat_map(|ability| ability.effects.iter().map(move |effect| (ability, effect)))
            .chain(
                pages()
                    .flat_map(|ability| ability.extras.iter().map(move |extra| (ability, extra))),
            )
            .collect()
    }

    /// Every effect of `key` as `page · ability · label · occurrence`, for errors
    fn candidates(&self, key: Key) -> String {
        let mut seen = BTreeMap::<&str, usize>::new();

        self.effects(key)
            .into_iter()
            .map(|(ability, effect)| {
                let occurrence = seen.entry(effect.label.as_str()).or_default();
                let line = format!(
                    "  {} · {} · {} · {}",
                    ability.page, ability.name, effect.label, occurrence
                );
                *occurrence += 1;
                line
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Takes the effect labeled `label` as `id`: the only one of `id`'s key, or of the
    /// ability named `ability`, or the `nth` one of the key
    fn select(
        &mut self,
        id: AbilityId,
        label: &str,
        ability: Option<&str>,
        nth: Option<usize>,
    ) -> MayFail<&mut Self> {
        let key = id.as_key();
        let champion_id = self.champion_id().to_string();
        let error = |this: &Self, problem: String| -> MayFail<&mut Self> {
            Err(format!(
                "[{champion_id}] {id:?}: {problem}. Effects of {key:?}:\n{}",
                this.candidates(key)
            )
            .into())
        };

        if self.damages.contains_key(&id) {
            return error(self, "taken already".into());
        }

        let matches = self
            .effects(key)
            .into_iter()
            .filter(|(model, effect)| {
                effect.label == label && ability.is_none_or(|name| model.name == name)
            })
            .collect::<Vec<_>>();

        let selected = match (nth, matches.len()) {
            (None, 1) => matches.first(),
            (None, 0) => None,
            (None, n) => {
                let problem = format!("{n} effects \"{label}\", use take_nth or take_in");
                return error(self, problem);
            }
            (Some(n), _) => matches.get(n),
        };

        let Some((model, effect)) = selected else {
            return error(self, format!("no effect \"{label}\""));
        };

        if !effect.issues.is_empty() {
            println!(
                "cargo:warning=[{champion_id}] {id:?} \"{label}\" has parts the model can't read: {:?}",
                effect.issues
            );
        }

        let damage = Damage {
            formula: effect.formula.clone(),
            name: model.name.clone(),
            damage_type: model.damage_type,
            comment: effect.label.clone(),
            attributes: Attrs::Undefined,
        };

        self.damages.insert(id, damage);
        Ok(self)
    }

    /// Formula of the only effect or description value labeled `label` among the pages
    /// of `key`, without taking it, as Caitlyn R's increase by critical strike chance
    pub fn value(&self, label: &str, key: Key) -> MayFail<Formula> {
        let matches = self
            .effects(key)
            .into_iter()
            .filter(|(_, effect)| effect.label == label)
            .collect::<Vec<_>>();

        match matches[..] {
            [(_, effect)] => Ok(effect.formula.clone()),
            _ => Err(format!(
                "[{}] {key:?}: {} values \"{label}\". Effects of {key:?}:\n{}",
                self.champion_id(),
                matches.len(),
                self.candidates(key)
            )
            .into()),
        }
    }

    /// Takes the only effect labeled `label` among every page of `id`'s key
    pub fn take(&mut self, label: &str, id: AbilityId) -> MayFail<&mut Self> {
        self.select(id, label, None, None)
    }

    /// Takes the only effect labeled `label` of the ability named `ability`, as
    /// Gnar's `"Boulder Toss"`
    pub fn take_in(&mut self, ability: &str, label: &str, id: AbilityId) -> MayFail<&mut Self> {
        self.select(id, label, Some(ability), None)
    }

    /// Takes the `nth` effect labeled `label` of `id`'s key, counted from 0 in page and
    /// tab order, as the cast and recast of Vex's R
    pub fn take_nth(&mut self, label: &str, nth: usize, id: AbilityId) -> MayFail<&mut Self> {
        self.select(id, label, None, Some(nth))
    }

    /// Adds `id` with `damage`, written by the generator: a damage no effect has, as
    /// the attack of Annie's Tibbers
    pub fn insert(&mut self, id: AbilityId, damage: Damage) -> MayFail<&mut Self> {
        if self.damages.contains_key(&id) {
            return Err(format!("[{}] {id:?}: taken already", self.champion_id()).into());
        }

        self.damages.insert(id, damage);
        Ok(self)
    }

    /// Damage `id`, which a generator must have taken before
    fn damage_mut(&mut self, id: AbilityId) -> &mut Damage {
        let champion_id = self.inner.champion_id.clone();

        self.damages
            .get_mut(&id)
            .unwrap_or_else(|| panic!("[{champion_id}] {id:?} was never taken"))
    }

    /// Replaces the formula of `id`
    pub fn set(&mut self, id: AbilityId, formula: Formula) -> &mut Self {
        self.damage_mut(id).formula = formula;
        self
    }

    /// Replaces the formula of `id` with `f` of it, applied to its value everywhere
    /// and on each mode that changes all of it
    pub fn edit(&mut self, id: AbilityId, f: impl Fn(Formula) -> Formula) -> &mut Self {
        let damage = self.damage_mut(id);
        damage.formula = damage.formula.clone().edit_arms(f);
        self
    }

    /// Adds `new` with `f` of the formula of `from`, and its name and damage type
    pub fn derive(
        &mut self,
        new: AbilityId,
        from: AbilityId,
        f: impl FnOnce(Formula) -> Formula,
    ) -> MayFail<&mut Self> {
        let champion_id = self.champion_id().to_string();

        if self.damages.contains_key(&new) {
            return Err(format!("[{champion_id}] {new:?}: taken already").into());
        }

        let mut damage = self
            .damages
            .get(&from)
            .ok_or_else(|| format!("[{champion_id}] {from:?} was never taken"))?
            .clone();

        damage.formula = f(damage.formula);
        self.damages.insert(new, damage);
        Ok(self)
    }

    pub fn comment(&mut self, id: AbilityId, comment: &str) -> &mut Self {
        self.damage_mut(id).comment = comment.to_string();
        self
    }

    pub fn damage_type(&mut self, id: AbilityId, damage_type: DamageType) -> &mut Self {
        self.damage_mut(id).damage_type = damage_type;
        self
    }

    pub fn attributes(&mut self, id: AbilityId, attributes: Attrs) -> &mut Self {
        self.damage_mut(id).attributes = attributes;
        self
    }

    /// How the formulas that game modes change are written, see [`MapStyle`]
    pub fn map_style(&mut self, style: MapStyle) -> &mut Self {
        self.style = style;
        self
    }

    /// A copy of the formula of `id`, for use in `f![…]`
    pub fn damage(&self, id: AbilityId) -> MayFail<Formula> {
        self.damages
            .get(&id)
            .map(|damage| damage.formula.clone())
            .ok_or_else(|| format!("[{}] {id:?} was never taken", self.champion_id()).into())
    }

    /// Description of the effect `effect` of `key` in v1's data, as v1's, for the
    /// constants generators read from it
    pub fn description(&self, key: Key, effect: &str) -> MayFail<&str> {
        self.inner.description(key, effect)
    }

    /// Text of one of the champion's wiki exceptions, as v1's
    pub fn exception(&self, name: &str) -> MayFail<&str> {
        self.inner.exception(name)
    }

    /// Adds `combos`, as v1. Every ability of a combo must have been taken before
    pub fn combos<const N: usize>(&mut self, combos: [&[ComboElement]; N]) -> MayFail<&mut Self> {
        for combo in combos {
            for element in combo {
                if let ComboElement::Ability(id) = element
                    && !self.damages.contains_key(id)
                {
                    let champion_id = self.champion_id();
                    return Err(format!("[{champion_id}] combo of {id:?}, never taken").into());
                }
            }

            self.inner.combo.push(combo.to_vec());
        }

        Ok(self)
    }

    pub fn constant(&mut self, name: &str, value: impl Into<Constant>) -> MayFail<&mut Self> {
        self.inner.constant(name, value)?;
        Ok(self)
    }

    /// Writes each damage into the abilities of [`Self::inner`], then runs v1's checks
    /// of `end`: zero damages are removed, unknown damage types warned and merges made.
    /// Under [`MapStyle::Value`], the damage and the codec's source are written from
    /// one tree. Under [`MapStyle::Whole`], each mode gets its whole formula, as v1
    pub fn end(&mut self) -> MayFail {
        for (id, damage) in std::mem::take(&mut self.damages) {
            let Damage {
                formula,
                name,
                damage_type,
                comment,
                attributes,
            } = damage;

            // v1 removes them in its `end`, from their text
            if formula.is_zero() {
                println!("[{}] Removing [{id:?}]", self.inner.champion_id);
                continue;
            }

            let rust = match self.style {
                MapStyle::Value => {
                    self.inner.docs.insert(id, formula.to_string());
                    formula.rust()
                }
                MapStyle::Whole => {
                    let modes = formula.modes();

                    if !modes.is_empty() {
                        let arms = modes
                            .into_iter()
                            .map(|mode| (mode, formula.on(mode).rust()))
                            .collect();

                        self.inner.map_damages.insert(id, arms);
                    }

                    formula.on(MapMode::Default).rust()
                }
            };

            let ability = Ability {
                name,
                damage_type,
                attributes,
                comment,
                damage: rust,
            };

            self.inner.abilities.insert(id, ability);
        }

        self.inner.end()?;

        // Damages that `end` removed have no source either
        let Champion {
            abilities, docs, ..
        } = &mut self.inner;
        docs.retain(|id, _| abilities.contains_key(id));

        Ok(())
    }

    /// The champion the build writes. Fails when damages were taken but never written,
    /// as by a generator that does not call [`Self::end`]
    pub fn into_champion(self) -> MayFail<Champion> {
        match self.damages.is_empty() {
            true => Ok(self.inner),
            false => Err(format!(
                "[{}] {:?} were taken but never written: the generator must call end()",
                self.champion_id(),
                self.damages.keys().collect::<Vec<_>>()
            )
            .into()),
        }
    }

    /// [`Self::end`], returning the champion, for tests and the diff
    pub fn finish(mut self) -> MayFail<Champion> {
        self.end()?;
        self.into_champion()
    }
}
