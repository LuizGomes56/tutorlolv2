use super::*;

impl Generator for Riftmaker {
    fn generate(&mut self) -> MayFail {
        // Void Corruption
        let [void_corruption_max_damage] = self
            .description(Named("Void Corruption"))?
            .capture(regex!(r"total of (\d+)% increased damage"))?;

        self.constant(
            "VOID_CORRUPTION_MAX_DAMAGE",
            void_corruption_max_damage / 100.0,
        )?;

        // Void Infusion
        let void_infusion = self.stat_ratio(Named("Void Infusion"))?;

        self.constant("VOID_INFUSION_BONUS_HEALTH_RATIO", void_infusion)?;

        Ok(())
    }
}
