use super::*;

impl Generator for RanduinsOmen {
    fn generate(&mut self) -> MayFail {
        // Resilience
        let [resilience_reduction] = self
            .description(Named("Resilience"))?
            .capture(regex!(r"critical strikes\}\} by (\d+)%"))?;

        self.constant("RESILIENCE_REDUCTION", resilience_reduction / 100.0)?;

        Ok(())
    }
}
