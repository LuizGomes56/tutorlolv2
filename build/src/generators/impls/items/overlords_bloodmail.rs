use super::*;

impl Generator for OverlordsBloodmail {
    fn generate(&mut self) -> MayFail {
        // Tyranny
        let [tyranny] = self
            .formula(Passive)?
            .capture(regex!(r"^\(\(([\d.]+)\) \* ctx\.bonus_health\)$"))?;

        self.constant("TYRANNY_BONUS_HEALTH_RATIO", tyranny)?;

        let mut tyranny = Vec::new();

        for (mode, formula) in self.map_formulas(Passive)? {
            let [value] = formula.capture(regex!(r"^\(\(([\d.]+)\) \* ctx\.bonus_health\)$"))?;
            tyranny.push((mode, value));
        }

        self.map_constant("TYRANNY_BONUS_HEALTH_RATIO", tyranny)?;

        // Retribution, which grows 1% at a time up to its maximum
        let [retribution_max_bonus, retribution_max_missing_health] =
            self.description(Passive2)?
                .capture(regex!(r"\{\{pp\|0 to (\d+) by 1\|0 to (\d+)\|"))?;

        self.constant("RETRIBUTION_MAX_BONUS", retribution_max_bonus / 100.0)?
            .constant("RETRIBUTION_STEPS", retribution_max_bonus)?
            .constant(
                "RETRIBUTION_MAX_MISSING_HEALTH",
                retribution_max_missing_health / 100.0,
            )?;

        // Arena reaches a larger bonus at the same missing health, growing every
        // few percent of missing health instead
        let (mut max_bonus, mut max_missing_health, mut steps) = (Vec::new(), Vec::new(), Vec::new());

        for (mode, text) in self.map_texts(regex!(r"^Retribution health ratio changed to"))? {
            let [bonus, missing_health, step] =
                text.capture(regex!(r"\{\{pp\|0 to ([\d.]+)\|0 to (\d+) by (\d+)"))?;

            max_bonus.push((mode, bonus / 100.0));
            max_missing_health.push((mode, missing_health / 100.0));
            steps.push((mode, missing_health / step));
        }

        self.map_constant("RETRIBUTION_MAX_BONUS", max_bonus)?
            .map_constant("RETRIBUTION_MAX_MISSING_HEALTH", max_missing_health)?
            .map_constant("RETRIBUTION_STEPS", steps)?;

        Ok(())
    }
}
