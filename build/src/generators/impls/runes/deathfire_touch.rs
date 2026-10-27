use super::*;

impl Generator for DeathfireTouch {
    fn generate(&mut self) -> MayFail {
        // Burn damage per tick, increased by 75% after lingering for 3 seconds
        let min_tick = self.compose([0, 1])?;
        let max_tick = format!("1.75 * ({min_tick})");

        self.assign_min(&min_tick)
            .assign_max(&max_tick)
            .damage_type(Adaptive)
            .end()
    }
}
