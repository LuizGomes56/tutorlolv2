use super::*;

impl Generator for PressTheAttack {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.damage_type(Physical)
            .min(0)? /* Adaptive Damage */
            .min(1)? /* Passive */
            .end()
    }
}
