use super::*;

impl Generator for ShieldBash {
    fn generate(&mut self) -> MayFail {
        self.min(0) /* Adaptive Damage */
            .min(1) /* Passive */
            .min(2) /* Passive [1] */
            .min(3) /* Passive [2] */
            .min(4) /* Passive [3] */
            .damage_type(Physical)
            .end()
    }
}
