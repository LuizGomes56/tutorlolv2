use super::*;

impl Generator for Riftmaker {
    fn generate(&mut self) -> MayFail {
        // Void Corruption
        let [void_corruption_max_damage] = self
            .description(Passive)?
            .capture(r"total of (\d+)% increased damage")?;

        self.constant(
            "VOID_CORRUPTION_MAX_DAMAGE",
            void_corruption_max_damage / 100.0,
        )?;

        Ok(())
    }
}
