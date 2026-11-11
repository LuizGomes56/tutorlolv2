use super::*;

impl Generator for RiteOfRuin {
    fn generate(&mut self) -> MayFail {
        // Wrath and Ruin, in percent like the critical strike chance of the engine
        let [wrath_and_ruin] = self
            .description(Passive)?
            .capture(regex!(r"grants \{\{sti\|\{\{fd\|([\d.]+)\}\}% critical strike chance\}\}"))?;

        self.constant("WRATH_AND_RUIN_CRIT_CHANCE_PER_STACK", wrath_and_ruin)?;

        Ok(())
    }
}
