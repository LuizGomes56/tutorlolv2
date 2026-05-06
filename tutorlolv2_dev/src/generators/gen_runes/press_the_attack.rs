use super::*;

impl Generator for PressTheAttack {
    fn generate(&mut self) -> MayFail {
        self.damage_type(Physical)
            .min(0)? /* Adaptive Damage */
            .min(1)? /* Passive */
            .end()
    }
}
