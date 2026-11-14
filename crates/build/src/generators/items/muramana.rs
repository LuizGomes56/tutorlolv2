use super::*;

impl Generator for Muramana {
    fn generate(&mut self) -> MayFail {
        // Awe
        let awe = self.stat_ratio(Named("Awe"))?;

        self.constant("AWE_MAX_MANA_RATIO", awe)?;

        Ok(())
    }
}
