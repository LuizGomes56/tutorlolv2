use super::*;

impl Generator for AbsorbLife {
    fn generate(&mut self) -> MayFail {
        self.min(0) /* Passive */
            .end()
    }
}
