use super::*;

impl Generator for SeraphsEmbrace {
    fn generate(&mut self) -> MayFail {
        // Awe
        let [awe] = self
            .formula(Passive)?
            .capture(regex!(r"^\(\(([\d.]+)\) \* ctx\.bonus_mana\)$"))?;

        self.constant("AWE_BONUS_MANA_RATIO", awe)?;

        Ok(())
    }
}
