use super::*;

impl Generator for Manamune {
    fn generate(&mut self) -> MayFail {
        // Awe
        let [awe] = self
            .formula(Passive)?
            .capture(regex!(r"^\(\(([\d.]+)\) \* ctx\.max_mana\)$"))?;

        self.constant("AWE_MAX_MANA_RATIO", awe)?;

        Ok(())
    }
}
