use super::*;

impl Generator for ShieldBash {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.damage_type(Physical)
            .min(0)? /* Adaptive Damage */
            .min(1)? /* Passive */
            .min(2)? /* Passive [1] */
            .min(3)? /* Passive [2] */
            .min(4)? /* Passive [3] */
            .end()
    }
}
