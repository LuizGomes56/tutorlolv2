use super::*;

impl Generator for ArcaneComet {
    fn generate(&mut self) -> MayFail {
        let damage = self.compose([0, 1])?;
        self.assign_min(&damage)
            .assign_max(f![(damage) * 2])
            .damage_type(Adaptive)
            .end()
    }
}
