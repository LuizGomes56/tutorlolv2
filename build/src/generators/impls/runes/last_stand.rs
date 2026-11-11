use super::*;

impl Generator for LastStand {
    fn generate(&mut self) -> MayFail {
        let [min_bonus_damage, min_health_threshold, max_bonus_damage, max_health_threshold] = self
            .description(0)?
            .capture(regex!(r"Deal (\d+)% increased damage to champions while below (\d+)% maximum health.*up to (\d+)% increased damage while below (\d+)% maximum health"))?;

        self.constant("MIN_BONUS_DAMAGE", min_bonus_damage / 100.0)?
            .constant("MIN_HEALTH_THRESHOLD", min_health_threshold / 100.0)?
            .constant("MAX_BONUS_DAMAGE", max_bonus_damage / 100.0)?
            .constant("MAX_HEALTH_THRESHOLD", max_health_threshold / 100.0)?;

        Ok(())
    }
}
