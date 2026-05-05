use super::*;

impl Generator for PressTheAttack {
    fn generate(&mut self) -> MayFail {
        self.min(0) /* Adaptive Damage */
            .min(1) /* Passive */
            .damage_type(Physical)
            .end()
    }
}
