use super::*;

impl Generator for PhaseRush {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.min(0)? /* Passive */
            .end()
    }
}
