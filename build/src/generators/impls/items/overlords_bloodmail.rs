use super::*;

impl Generator for OverlordsBloodmail {
    fn generate(&mut self) -> MayFail {
        // Retribution
        let [retribution_max_bonus, retribution_max_missing_health] =
            self.description(Passive2)?
                .capture(r"\{\{pp\|0 to (\d+) by 1\|0 to (\d+)\|")?;

        self.constant("RETRIBUTION_MAX_BONUS", retribution_max_bonus / 100.0)?
            .constant(
                "RETRIBUTION_MAX_MISSING_HEALTH",
                retribution_max_missing_health / 100.0,
            )?;

        Ok(())
    }
}
