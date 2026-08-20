use crate::{
    AbilityId, ChampionId, DamageIndex, ItemId, RuneId,
    yew::render::{MayFail, RENDERER, Renderer},
};
use alloc::{format, string::String};
use tutorlolv2_codec::{Class, Highlighter};
use tutorlolv2_types::AttackType;

pub const TYPE_INDEX: [(AttackType, DamageIndex); 4] = [
    (AttackType::Melee, DamageIndex::Min),
    (AttackType::Melee, DamageIndex::Max),
    (AttackType::Ranged, DamageIndex::Min),
    (AttackType::Ranged, DamageIndex::Max),
];

impl ChampionId {
    pub fn render_global(&self) -> MayFail<String> {
        let mut h = Highlighter::new();

        h.global_struct(self.debug())
            .field("name", Class::String, self.name())
            .field("adaptive_type", Class::Constant, self.adaptive_type())
            .field("attack_type", Class::Constant, self.attack_type())
            .array_field("positions", Class::Type, self.positions());

        for metadata in self.abilities() {
            let name = metadata.kind.discriminant().to_lowercase();
            let damage = RENDERER.champion_formula(*self, metadata.kind)?;
            h.html_field(&name, &damage);
        }

        h.finish_struct();

        Ok(h.into_fragment())
    }

    pub fn render_fn(&self, ability_id: AbilityId) -> MayFail<String> {
        let mut h = Highlighter::new();

        let body = RENDERER.champion_formula(*self, ability_id)?;

        h.function(
            self.debug(),
            ability_id.as_char(),
            ability_id.ability_name(),
            &body,
        );

        Ok(h.into_fragment())
    }
}

impl ItemId {
    pub fn render_fn(&self) -> MayFail<String> {
        let mut h = Highlighter::new();

        if !self.deals_damage() {
            h.function(
                self.debug(),
                "Type",
                "Tag",
                &Highlighter::span(Class::Comment, "No damage"),
            );

            return Ok(h.into_fragment());
        }

        for (attack_type, damage_index) in TYPE_INDEX {
            if !self.deals_max_damage() && matches!(damage_index, DamageIndex::Max) {
                continue;
            }

            let slot = Renderer::damage_slot(attack_type, damage_index);
            let body = RENDERER.cast_id_formula(*self, slot)?;

            h.function(self.debug(), attack_type, damage_index, &body)
                .raw("\n\n");
        }

        Ok(h.into_fragment())
    }

    pub fn render_global(&self) -> MayFail<String> {
        let mut h = Highlighter::new();

        h.global_struct(self.debug())
            .field("name", Class::String, self.name())
            .field("price", Class::Number, self.price())
            .tuple_field("stats", Class::Number, self.stats())
            .field("tier", Class::Number, self.tier())
            .field("purchasable", Class::Boolean, self.purchasable())
            .array_field("maps", Class::Constant, self.maps());

        if self.deals_damage() {
            for (attack_type, damage_index) in TYPE_INDEX {
                if !self.deals_max_damage() && matches!(damage_index, DamageIndex::Max) {
                    continue;
                }

                let slot = Renderer::damage_slot(attack_type, damage_index);
                let name = format!("{attack_type:?}_{damage_index:?}").to_lowercase();
                let damage = RENDERER.cast_id_formula(*self, slot)?;

                h.html_field(&name, &damage);
            }
        }

        h.finish_struct();

        Ok(h.into_fragment())
    }
}

impl RuneId {
    pub fn render_fn(&self) -> MayFail<String> {
        let mut h = Highlighter::new();

        if !self.deals_damage() {
            h.function(
                self.debug(),
                "Type",
                "Tag",
                &Highlighter::span(Class::Comment, "No damage"),
            );

            return Ok(h.into_fragment());
        }

        for (attack_type, damage_index) in TYPE_INDEX {
            if !self.deals_max_damage() && matches!(damage_index, DamageIndex::Max) {
                continue;
            }

            let slot = Renderer::damage_slot(attack_type, damage_index);
            let body = RENDERER.cast_id_formula(*self, slot)?;

            h.function(self.debug(), attack_type, damage_index, &body)
                .raw("\n\n");
        }

        Ok(h.into_fragment())
    }

    pub fn render_global(&self) -> MayFail<String> {
        let mut h = Highlighter::new();

        h.global_struct(self.debug())
            .field("name", Class::String, self.name());

        if self.deals_damage() {
            for (attack_type, damage_index) in TYPE_INDEX {
                if !self.deals_max_damage() && matches!(damage_index, DamageIndex::Max) {
                    continue;
                }

                let slot = Renderer::damage_slot(attack_type, damage_index);
                let name = format!("{attack_type:?}_{damage_index:?}").to_lowercase();
                let damage = RENDERER.cast_id_formula(*self, slot)?;

                h.html_field(&name, &damage);
            }
        }

        h.finish_struct();

        Ok(h.into_fragment())
    }
}
