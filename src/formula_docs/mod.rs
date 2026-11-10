mod builtin;
mod ext;

use crate::{AbilityId, AttackType, CastId, ChampionId, CtxVar, DamageIndex};
use alloc::{
    borrow::ToOwned,
    boxed::Box,
    string::{String, ToString},
};
use tutorlolv2_codec::{DamageSlot, FormulaDb, GeneratorDb, Html, Markup, render::FnBuilder};

pub type MayFail<T = ()> = Result<T, Box<dyn core::error::Error>>;

static PACKED_FORMULAS: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/formulas.bin"));
static PACKED_GENERATORS: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/generator.bin"));

#[derive(Clone, Copy)]
pub struct GeneratorRenderer {
    db: GeneratorDb<'static>,
}

impl GeneratorRenderer {
    pub const fn new() -> Self {
        Self {
            db: match GeneratorDb::new(PACKED_GENERATORS) {
                Ok(db) => db,
                Err(_) => panic!("invalid embedded generator database"),
            },
        }
    }

    pub fn render<T: CastId>(&self, value: T) -> Option<String> {
        self.render_markup::<Html, T>(value).map(Html::into_string)
    }

    pub fn render_markup<M: Markup, T: CastId>(&self, value: T) -> Option<M> {
        let index = value.index();

        unsafe {
            self.db
                .render_markup(T::KIND, index as _)
                .unwrap_unchecked()
        }
    }
}

impl Default for GeneratorRenderer {
    fn default() -> Self {
        Self::new()
    }
}

pub static GENERATOR_RENDERER: GeneratorRenderer = GeneratorRenderer::new();
pub static FORMULA_RENDERER: FormulaRenderer = FormulaRenderer::new();

#[derive(Clone, Copy)]
pub struct FormulaRenderer {
    db: FormulaDb<'static>,
}

impl FormulaRenderer {
    pub const fn new() -> Self {
        Self {
            db: match FormulaDb::new(PACKED_FORMULAS) {
                Ok(db) => db,
                Err(_) => panic!("invalid embedded formula database"),
            },
        }
    }

    pub fn champion_formula(&self, champion: ChampionId, ability_id: AbilityId) -> MayFail<String> {
        self.champion_formula_markup::<Html>(champion, ability_id)
            .map(Html::into_string)
    }

    pub fn champion_formula_markup<M: Markup>(
        &self,
        champion: ChampionId,
        ability_id: AbilityId,
    ) -> MayFail<M> {
        let local = champion
            .index_of_ability(ability_id)
            .ok_or("Unable to locate ability_id")? as u8;

        let formula_id = self
            .db
            .champion_formula_id(champion as _, local)
            .ok_or("Unable to locate formula_id")?;

        Ok(self
            .db
            .render_formula_markup(formula_id, Self::ctx_name, |local| {
                let ability = &champion.abilities()[local as usize].kind;

                FnBuilder {
                    fn_struct: champion.debug().to_string(),
                    fn_type: ability.as_char().to_string(),
                    fn_tag: <&str>::from(ability.ability_name()).to_string(),
                }
            })?)
    }

    pub fn cast_id_formula<T: CastId>(&self, id: T, slot: DamageSlot) -> MayFail<String> {
        self.cast_id_formula_markup::<Html, T>(id, slot)
            .map(Html::into_string)
    }

    pub fn cast_id_formula_markup<M: Markup, T: CastId>(
        &self,
        id: T,
        slot: DamageSlot,
    ) -> MayFail<M> {
        let formula_id = self
            .db
            .item_or_rune_formula_id(T::KIND, id.index() as _, slot)
            .ok_or("Unable to locate formula_id")?;

        let fn_struct = id.debug();

        Ok(self
            .db
            .render_formula_markup(formula_id, Self::ctx_name, |local| {
                let (fn_type, fn_tag) = match DamageSlot::from_u8(local) {
                    Some(DamageSlot::MeleeMin) => ("Melee", "Min"),
                    Some(DamageSlot::MeleeMax) => ("Melee", "Max"),
                    Some(DamageSlot::RangedMin) => ("Ranged", "Min"),
                    Some(DamageSlot::RangedMax) => ("Ranged", "Max"),
                    None => ("Type", "Tag"),
                };

                FnBuilder {
                    fn_struct: fn_struct.to_string(),
                    fn_type: fn_type.to_string(),
                    fn_tag: fn_tag.to_string(),
                }
            })?)
    }

    fn ctx_name(ctx: u8) -> String {
        CtxVar::from_repr(ctx)
            .map_or("unknown", |value| value.to_snake_case())
            .to_owned()
    }

    const fn damage_slot(attack_type: AttackType, damage_index: DamageIndex) -> DamageSlot {
        DamageSlot::from_u8(2 * attack_type as u8 + damage_index as u8).unwrap()
    }
}

impl Default for FormulaRenderer {
    fn default() -> Self {
        Self::new()
    }
}
