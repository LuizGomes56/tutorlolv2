use super::*;

impl Generator for AxiomArcanist {
    fn generate(&mut self) -> MayFail {
        let [ultimate_bonus_damage, area_ultimate_bonus_damage] = self
            .description(0)?
            .capture(regex!(r"has (\d+)% increased damage \(reduced to (\d+)% for area"))?;

        self.constant("ULTIMATE_BONUS_DAMAGE", ultimate_bonus_damage / 100.0)?
            .constant(
                "AREA_ULTIMATE_BONUS_DAMAGE",
                area_ultimate_bonus_damage / 100.0,
            )?;

        Ok(())
    }
}
