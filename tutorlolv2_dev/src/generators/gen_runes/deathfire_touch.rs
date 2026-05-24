use super::*;

impl Generator for DeathfireTouch {
    fn generate(&mut self) -> MayFail {
        let min_tick = self.compose([1, 2])?;
        let max_tick = self.compose([6, 3, 4])?;

        self.assign_min(&min_tick)
            .assign_max(&max_tick)
            .damage_type(Adaptive)
            .end()
    }
}
