use super::*;

impl Generator for SeraphsEmbrace {
    fn generate(&mut self) -> MayFail {
        // Awe
        let awe = self.effect(Passive)?.scalings[0]
            .try_get_value()
            .ok_or("Expected a constant stat scaling")?;

        self.constant("AWE_BONUS_MANA_RATIO", awe)?;

        // Awe on the modes that change it
        let mut awe = Vec::new();

        for (mode, effect) in self.map_effects_for(Passive, Melee)? {
            let value = effect.scalings[0]
                .try_get_value()
                .ok_or("Expected a constant map stat scaling")?;

            awe.push((mode, value));
        }

        self.map_constant("AWE_BONUS_MANA_RATIO", awe)?;

        Ok(())
    }
}
