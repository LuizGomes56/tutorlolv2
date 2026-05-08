use super::*;

impl Generator for SuddenImpact {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.damage_type(True)
            .min(0)? /* Passive */
            .end()
    }
}
