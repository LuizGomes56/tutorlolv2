use super::*;

impl Generator for SpearOfShojin {
    fn generate(&mut self) -> MayFail {
        // Focused Will
        let [focused_will_damage_per_stack, focused_will_max_damage] = self
            .description(Passive2)?
            .capture(r"deals (\d+)% increased damage, for a total increase of (\d+)%")?;

        self.constant(
            "FOCUSED_WILL_DAMAGE_PER_STACK",
            focused_will_damage_per_stack / 100.0,
        )?
        .constant("FOCUSED_WILL_MAX_DAMAGE", focused_will_max_damage / 100.0)?;

        Ok(())
    }
}
