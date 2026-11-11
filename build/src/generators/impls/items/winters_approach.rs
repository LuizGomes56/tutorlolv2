use super::*;

impl Generator for WintersApproach {
    fn generate(&mut self) -> MayFail {
        // Awe
        let [awe] = self
            .formula(Passive)?
            .capture(regex!(r"^\(\(([\d.]+)\) \* ctx\.bonus_mana\)$"))?;

        self.constant("AWE_BONUS_MANA_RATIO", awe)?;

        // Awe on the modes that change it
        let mut awe = Vec::new();

        for (mode, formula) in self.map_formulas(Passive)? {
            let [value] = formula.capture(regex!(r"^\(\(([\d.]+)\) \* ctx\.bonus_mana\)$"))?;
            awe.push((mode, value));
        }

        self.map_constant("AWE_BONUS_MANA_RATIO", awe)?;

        Ok(())
    }
}
