use crate::{
    AbilityId, CastId, ChampionId, DamageIndex, ItemId, RuneId,
    formula_docs::{FORMULA_RENDERER, FormulaRenderer, MayFail},
};
use alloc::{format, string::String};
use codec::{Bracket, Class, GenericHighlighter, Html, Markup};
use types::{AttackType, ComboElement};

pub const TYPE_INDEX: [(AttackType, DamageIndex); 4] = [
    (AttackType::Melee, DamageIndex::Min),
    (AttackType::Melee, DamageIndex::Max),
    (AttackType::Ranged, DamageIndex::Min),
    (AttackType::Ranged, DamageIndex::Max),
];

impl ChampionId {
    pub fn render_global_markup<M: Markup>(&self) -> MayFail<M> {
        let mut h = GenericHighlighter::<M>::new();

        h.global_struct(self.debug())
            .field("name", Class::String, self.name())
            .field("adaptive_type", Class::Constant, self.adaptive_type())
            .field("attack_type", Class::Constant, self.attack_type())
            .array_field("positions", Class::Type, self.positions());

        for metadata in self.abilities() {
            let name = metadata.kind.discriminant().to_lowercase();
            let damage = FORMULA_RENDERER.champion_formula_markup(*self, metadata.kind)?;
            h.fragment_field(&name, &damage);
        }

        for (i, combo) in self.combos().iter().enumerate() {
            let values = combo.iter().map(|c| match c {
                ComboElement::Ability(ability_id) => {
                    let mut h = GenericHighlighter::<M>::new();

                    h.push(Class::Type, ability_id.as_char())
                        .bracket(Bracket::LParen)
                        .push(Class::Constant, ability_id.ability_name())
                        .bracket(Bracket::RParen);

                    h.into_markup()
                }
                ComboElement::Attack => {
                    GenericHighlighter::<M>::span_markup(Class::Constant, "Attack")
                }
            });

            let name = format!("combo_{}", i + 1);

            h.fragment_array_field(&name, values);
        }

        h.finish_struct();

        Ok(h.into_markup())
    }

    pub fn render_fn_markup<M: Markup>(&self, ability_id: AbilityId) -> MayFail<M> {
        let mut h = GenericHighlighter::<M>::new();

        let body = FORMULA_RENDERER.champion_formula_markup(*self, ability_id)?;

        h.function_fragment(
            self.debug(),
            ability_id.as_char(),
            ability_id.ability_name(),
            &body,
        );

        Ok(h.into_markup())
    }
}

impl ItemId {
    pub fn render_fn_markup<M: Markup>(&self) -> MayFail<M> {
        let mut h = GenericHighlighter::<M>::new();

        if !self.deals_damage() {
            h.function_fragment(
                self.debug(),
                "Type",
                "Tag",
                &GenericHighlighter::<M>::span_markup(Class::Comment, "No damage"),
            );

            return Ok(h.into_markup());
        }

        for (attack_type, damage_index) in TYPE_INDEX {
            if !self.deals_max_damage() && matches!(damage_index, DamageIndex::Max) {
                continue;
            }

            let slot = FormulaRenderer::damage_slot(attack_type, damage_index);
            let body = FORMULA_RENDERER.cast_id_formula_markup(*self, slot)?;

            h.function_fragment(self.debug(), attack_type, damage_index, &body)
                .raw("\n\n");
        }

        Ok(h.into_markup())
    }

    pub fn render_global_markup<M: Markup>(&self) -> MayFail<M> {
        let mut h = GenericHighlighter::<M>::new();

        h.global_struct(self.debug())
            .field("name", Class::String, self.name())
            .field("price", Class::Number, self.price())
            .tuple_field("stats", Class::Number, self.stats())
            .field("tier", Class::Number, self.tier())
            .field("purchasable", Class::Boolean, self.purchasable())
            .array_field("maps", Class::Constant, self.maps());

        if self.deals_damage() {
            h.field("damage_type", Class::Constant, self.damage_type());

            for (attack_type, damage_index) in TYPE_INDEX {
                if !self.deals_max_damage() && matches!(damage_index, DamageIndex::Max) {
                    continue;
                }

                let slot = FormulaRenderer::damage_slot(attack_type, damage_index);
                let name = format!("{attack_type:?}_{damage_index:?}").to_lowercase();
                let damage = FORMULA_RENDERER.cast_id_formula_markup(*self, slot)?;

                h.fragment_field(&name, &damage);
            }
        }

        h.array_field(
            "recipe",
            Class::Type,
            self.builds_from().iter().map(Self::debug),
        )
        .array_field(
            "builds",
            Class::Type,
            self.builds_into().iter().map(Self::debug),
        )
        .finish_struct();

        Ok(h.into_markup())
    }
}

impl RuneId {
    pub fn render_fn_markup<M: Markup>(&self) -> MayFail<M> {
        let mut h = GenericHighlighter::<M>::new();

        if !self.deals_damage() {
            h.function_fragment(
                self.debug(),
                "Type",
                "Tag",
                &GenericHighlighter::<M>::span_markup(Class::Comment, "No damage"),
            );

            return Ok(h.into_markup());
        }

        for (attack_type, damage_index) in TYPE_INDEX {
            if !self.deals_max_damage() && matches!(damage_index, DamageIndex::Max) {
                continue;
            }

            let slot = FormulaRenderer::damage_slot(attack_type, damage_index);
            let body = FORMULA_RENDERER.cast_id_formula_markup(*self, slot)?;

            h.function_fragment(self.debug(), attack_type, damage_index, &body)
                .raw("\n\n");
        }

        Ok(h.into_markup())
    }

    pub fn render_global_markup<M: Markup>(&self) -> MayFail<M> {
        let mut h = GenericHighlighter::<M>::new();

        h.global_struct(self.debug())
            .field("name", Class::String, self.name())
            .field("riot_id", Class::Number, self.riot_id());

        if self.deals_damage() {
            h.field("damage_type", Class::Constant, self.damage_type());

            for (attack_type, damage_index) in TYPE_INDEX {
                if !self.deals_max_damage() && matches!(damage_index, DamageIndex::Max) {
                    continue;
                }

                let slot = FormulaRenderer::damage_slot(attack_type, damage_index);
                let name = format!("{attack_type:?}_{damage_index:?}").to_lowercase();
                let damage = FORMULA_RENDERER.cast_id_formula_markup(*self, slot)?;

                h.fragment_field(&name, &damage);
            }
        }

        h.finish_struct();

        Ok(h.into_markup())
    }
}

// HTML strings, as used by the web frontend.

impl ChampionId {
    pub fn render_global(&self) -> MayFail<String> {
        self.render_global_markup::<Html>().map(Html::into_string)
    }

    pub fn render_fn(&self, ability_id: AbilityId) -> MayFail<String> {
        self.render_fn_markup::<Html>(ability_id)
            .map(Html::into_string)
    }
}

impl ItemId {
    pub fn render_fn(&self) -> MayFail<String> {
        self.render_fn_markup::<Html>().map(Html::into_string)
    }

    pub fn render_global(&self) -> MayFail<String> {
        self.render_global_markup::<Html>().map(Html::into_string)
    }
}

impl RuneId {
    pub fn render_fn(&self) -> MayFail<String> {
        self.render_fn_markup::<Html>().map(Html::into_string)
    }

    pub fn render_global(&self) -> MayFail<String> {
        self.render_global_markup::<Html>().map(Html::into_string)
    }
}
