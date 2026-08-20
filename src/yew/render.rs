use crate::{
    AbilityId, AttackType, CastId, ChampionId, CtxVar, DamageIndex, EntityId, ItemId, RuneId,
};
use alloc::{
    borrow::ToOwned,
    boxed::Box,
    format,
    string::{String, ToString},
};
use tutorlolv2_codec::{
    Class, DamageSlot, EntityKind, Error, FormulaDb, Highlighter, render::FnBuilder,
};

pub type MayFail<T = ()> = Result<T, Box<dyn core::error::Error>>;

const TYPE_INDEX: [(AttackType, DamageIndex); 4] = [
    (AttackType::Melee, DamageIndex::Min),
    (AttackType::Melee, DamageIndex::Max),
    (AttackType::Ranged, DamageIndex::Min),
    (AttackType::Ranged, DamageIndex::Max),
];

pub struct Renderer<'a> {
    db: FormulaDb<'a>,
}

impl<'a> Renderer<'a> {
    pub fn parse(bytes: &'a [u8]) -> Result<Self, Error> {
        Ok(Self {
            db: FormulaDb::parse(bytes)?,
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
            .sparse_formula_id(entity, id.index() as _, slot)
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

    pub fn champion_function(
        &self,
        champion: ChampionId,
        ability_id: AbilityId,
    ) -> MayFail<String> {
        let mut h = Highlighter::new();

        let body = self.champion_formula(champion, ability_id)?;

        h.function(
            champion.debug(),
            ability_id.as_char(),
            ability_id.ability_name(),
            &body,
        );

        Ok(h.into_fragment())
    }

    pub fn champion_global(&self, champion: ChampionId) -> MayFail<String> {
        let mut h = Highlighter::new();

        h.global_struct(champion.debug())
            .field("name", Class::String, champion.name())
            .field("adaptive_type", Class::Constant, champion.adaptive_type())
            .field("attack_type", Class::Constant, champion.attack_type())
            .array_field("positions", Class::Type, champion.positions());

        for metadata in champion.abilities() {
            let name = metadata.kind.discriminant().to_lowercase();
            let damage = self.champion_formula(champion, metadata.kind)?;
            h.html_field(&name, &damage);
        }

        h.finish_struct();

        Ok(h.into_fragment())
    }

    pub fn item_function(&self, item: ItemId) -> MayFail<String> {
        let mut h = Highlighter::new();

        if !item.deals_damage() {
            h.function(
                item.debug(),
                "Type",
                "Tag",
                &Highlighter::span(Class::Comment, "No damage"),
            );

            return Ok(h.into_fragment());
        }

        for (attack_type, damage_index) in TYPE_INDEX {
            if !item.deals_max_damage() && matches!(damage_index, DamageIndex::Max) {
                continue;
            }

            let slot = Self::damage_slot(attack_type, damage_index);
            let body = self.cast_id_formula(item, slot)?;

            h.function(item.debug(), attack_type, damage_index, &body)
                .raw("\n\n");
        }

        Ok(h.into_fragment())
    }

    pub fn item_global(&self, item: ItemId) -> MayFail<String> {
        let mut h = Highlighter::new();

        h.global_struct(item.debug())
            .field("name", Class::String, item.name())
            .field("price", Class::Number, item.price())
            .tuple_field("stats", Class::Number, item.stats())
            .field("tier", Class::Number, item.tier())
            .field("purchasable", Class::Boolean, item.purchasable())
            .array_field("maps", Class::Constant, item.maps());

        if item.deals_damage() {
            for (attack_type, damage_index) in TYPE_INDEX {
                if !item.deals_max_damage() && matches!(damage_index, DamageIndex::Max) {
                    continue;
                }

                let slot = Self::damage_slot(attack_type, damage_index);
                let name = format!("{attack_type:?}_{damage_index:?}").to_lowercase();
                let damage = self.cast_id_formula(item, slot)?;

                h.html_field(&name, &damage);
            }
        }

        h.finish_struct();

        Ok(h.into_fragment())
    }

    pub fn rune_function(&self, rune: RuneId) -> MayFail<String> {
        let mut h = Highlighter::new();

        if !rune.deals_damage() {
            h.function(
                rune.debug(),
                "Type",
                "Tag",
                &Highlighter::span(Class::Comment, "No damage"),
            );

            return Ok(h.into_fragment());
        }

        for (attack_type, damage_index) in TYPE_INDEX {
            if !rune.deals_max_damage() && matches!(damage_index, DamageIndex::Max) {
                continue;
            }

            let slot = Self::damage_slot(attack_type, damage_index);
            let body = self.cast_id_formula(rune, slot)?;

            h.function(rune.debug(), attack_type, damage_index, &body)
                .raw("\n\n");
        }

        Ok(h.into_fragment())
    }

    pub fn rune_global(&self, rune: RuneId) -> MayFail<String> {
        let mut h = Highlighter::new();

        h.global_struct(rune.debug())
            .field("name", Class::String, rune.name());

        if rune.deals_damage() {
            for (attack_type, damage_index) in TYPE_INDEX {
                if !rune.deals_max_damage() && matches!(damage_index, DamageIndex::Max) {
                    continue;
                }

                let slot = Self::damage_slot(attack_type, damage_index);
                let name = format!("{attack_type:?}_{damage_index:?}").to_lowercase();
                let damage = self.cast_id_formula(rune, slot)?;

                h.html_field(&name, &damage);
            }
        }

        h.finish_struct();

        Ok(h.into_fragment())
    }

    fn ctx_name(ctx: u8) -> String {
        let raw = CtxVar::from_repr(ctx)
            .map(|value| value.as_var())
            .unwrap_or("ctx.unknown");

        raw.strip_prefix("ctx.").unwrap_or(raw).to_owned()
    }

    fn damage_slot(attack_type: AttackType, damage_index: DamageIndex) -> DamageSlot {
        DamageSlot::from_u8(2 * attack_type as u8 + damage_index as u8)
            .expect("AttackType/DamageIndex mapping must fit DamageSlot")
    }
}
