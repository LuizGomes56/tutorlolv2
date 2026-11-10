use super::*;

impl Generator for PlatedSteelcaps {
    fn generate(&mut self) -> MayFail {
        // Plating
        let [plating_reduction] = self
            .description(Passive)?
            .capture(r"basic damage\}\} by (\d+)%")?;

        self.constant("PLATING_REDUCTION", plating_reduction / 100.0)?;

        Ok(())
    }
}
