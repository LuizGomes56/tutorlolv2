use super::*;

impl Generator for DemonicEmbrace {
    fn generate(&mut self) -> MayFail {
        // Sinister Pact
        let [
            sinister_pact_max_ap,
            sinister_pact_max_missing_health,
            sinister_pact_health_step,
        ] = self
            .description(Passive)?
            .capture(regex!(r"\{\{pp\|0 to (\d+)\|0 to (\d+) by (\d+)\|"))?;

        self.constant("SINISTER_PACT_MAX_AP", sinister_pact_max_ap)?
            .constant(
                "SINISTER_PACT_MAX_MISSING_HEALTH",
                sinister_pact_max_missing_health,
            )?
            .constant("SINISTER_PACT_HEALTH_STEP", sinister_pact_health_step)?;

        Ok(())
    }
}
