use super::*;

impl Generator for SeraphsEmbrace {
    fn generate(&mut self) -> MayFail {
        // Awe
        let awe = self.stat_ratio(Named("Awe"))?;

        self.constant("AWE_BONUS_MANA_RATIO", awe)?;

        // Awe on the modes that change it
        let mut awe = Vec::new();

        for (mode, value) in self.map_stat_ratios(Named("Awe"))? {
            awe.push((mode, value));
        }

        self.map_constant("AWE_BONUS_MANA_RATIO", awe)?;

        Ok(())
    }
}
