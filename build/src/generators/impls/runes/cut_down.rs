use super::*;

impl Generator for CutDown {
    fn generate(&mut self) -> MayFail {
        let [bonus_damage, health_threshold] = self
            .description(0)?
            .capture(r"Deal (\d+)% increased damage to champions above (\d+)% maximum health")?;

        self.constant("BONUS_DAMAGE", bonus_damage / 100.0)?
            .constant("HEALTH_THRESHOLD", health_threshold / 100.0)?;

        Ok(())
    }
}
