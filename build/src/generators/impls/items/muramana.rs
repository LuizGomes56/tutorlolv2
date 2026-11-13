use super::*;

impl Generator for Muramana {
    fn generate(&mut self) -> MayFail {
        // Awe
        let awe = self.effect(Passive)?.scalings[0]
            .try_get_value()
            .ok_or("Expected a constant stat scaling")?;

        self.constant("AWE_MAX_MANA_RATIO", awe)?;

        Ok(())
    }
}
