use super::*;

impl Generator for ArmoredAdvance {
    fn generate(&mut self) -> MayFail {
        // Plating
        let [plating_reduction] = self
            .description(Named("Plating"))?
            .capture(regex!(r"basic damage\}\} by (\d+)%"))?;

        self.constant("PLATING_REDUCTION", plating_reduction / 100.0)?;

        Ok(())
    }
}
