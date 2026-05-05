use super::*;

impl Generator for PhaseRush {
    fn generate(&mut self) -> MayFail {
        self.min(0)? /* Passive */
            .end()
    }
}
