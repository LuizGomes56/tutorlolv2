use super::*;

impl Generator for BonePlating {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.damage_type(True)
            .min(0)? /* Passive */
            .end()
    }
}
