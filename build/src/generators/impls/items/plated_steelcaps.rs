use super::*;

impl Generator for PlatedSteelcaps {
    fn generate(&mut self) -> MayFail {
        // Plating
        let [plating_reduction] = self
            .description(Passive)?
            .capture(regex!(r"basic damage\}\} by (\d+)%"))?;

        self.constant("PLATING_REDUCTION", plating_reduction / 100.0)?;

        let mut plating = Vec::new();

        for (mode, amount) in self.map_values("Plating damage reduction")? {
            let value = amount
                .single(false)
                .ok_or("Plating damage reduction is not a single value")?;

            plating.push((mode, value / 100.0));
        }

        self.map_constant("PLATING_REDUCTION", plating)?;

        Ok(())
    }
}
