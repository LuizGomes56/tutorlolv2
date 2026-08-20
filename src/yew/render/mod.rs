mod ext;
mod tower;

use crate::{AbilityId, AttackType, CastId, ChampionId, CtxVar, DamageIndex, EntityId};
use alloc::{
    borrow::ToOwned,
    boxed::Box,
    string::{String, ToString},
};
use std::sync::LazyLock;
use tutorlolv2_codec::{DamageSlot, EntityKind, Error, FormulaDb, render::FnBuilder};

pub const IGNITE_FN: &str = r#"fn ignite(level: i32) -> i32 {
    70 + 20 * level + 5
      * if level > 4 { level - 4 }
        else { 0 }
}"#;

pub const ONHIT_EFFECT: &str = r#"intrinsic Onhit {
    damage_type: Mixed,
    definition: fn onhit(...) -> Attacks
};"#;

pub const ONHIT_EFFECT_FN: &str = r#"fn onhit() -> Attacks {
    intrinsic
}"#;

pub const CRITICAL_STRIKE: &str = r#"intrinsic CritStrike {
    attributes: OnhitMax,
    damage_type: Physical,
    damage: attack_damage * crit_damage / 100
};"#;

pub const CRITICAL_STRIKE_FN: &str = r#"fn critical_strike() -> f32 {
    attack_damage * crit_damage
      / 100 /* * physical_multiplier */
}"#;

pub const BASIC_ATTACK: &str = r#"intrinsic BasicAttack {
    attributes: OnhitMin,
    damage_type: Physical,
    damage: attack_damage /* * physical_multiplier */,
};"#;

pub const BASIC_ATTACK_FN: &str = r#"fn basic_attack() -> f32 {
    attack_damage /* * physical_multiplier */
}"#;

pub type MayFail<T = ()> = Result<T, Box<dyn core::error::Error>>;

static PACKED: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/packer.bin"));
pub static RENDERER: LazyLock<Renderer> = LazyLock::new(|| Renderer::new().unwrap());

pub struct Renderer {
    db: FormulaDb<'static>,
}

impl Renderer {
    pub fn new() -> Result<Self, Error> {
        Ok(Self {
            db: FormulaDb::parse(PACKED)?,
        })
    }

    pub fn champion_formula(&self, champion: ChampionId, ability_id: AbilityId) -> MayFail<String> {
        let local = champion
            .index_of_ability(ability_id)
            .ok_or("Unable to locate ability_id")? as u8;

        let formula_id = self
            .db
            .champion_formula_id(champion as _, local)
            .ok_or("Unable to locate formula_id")?;

        Ok(self
            .db
            .render_formula_html(formula_id, Self::ctx_name, |local| {
                let ability = &champion.abilities()[local as usize].kind;

                FnBuilder {
                    fn_struct: champion.debug().to_string(),
                    fn_type: ability.as_char().to_string(),
                    fn_tag: <&str>::from(ability.ability_name()).to_string(),
                }
            })?)
    }

    pub fn cast_id_formula(&self, id: impl CastId, slot: DamageSlot) -> MayFail<String> {
        let entity = match id.entity() {
            EntityId::Champion(_) => EntityKind::Champion,
            EntityId::Item(_) => EntityKind::Item,
            EntityId::Rune(_) => EntityKind::Rune,
        };

        let formula_id = self
            .db
            .item_or_rune_formula_id(entity, id.index() as _, slot)
            .ok_or("Unable to locate formula_id")?;

        let fn_struct = id.debug();

        Ok(self
            .db
            .render_formula_html(formula_id, Self::ctx_name, |local| {
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
        let raw = CtxVar::from_repr(ctx)
            .map(|value| value.as_var())
            .unwrap_or("ctx.unknown");

        raw.strip_prefix("ctx.").unwrap_or(raw).to_owned()
    }

    fn damage_slot(attack_type: AttackType, damage_index: DamageIndex) -> DamageSlot {
        DamageSlot::from_u8(2 * attack_type as u8 + damage_index as u8).unwrap()
    }
}
