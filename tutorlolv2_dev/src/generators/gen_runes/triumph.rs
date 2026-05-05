use super::*;

impl Generator for Triumph {
    fn generate(&mut self) -> MayFail {
        self.min(0) /* Passive */
            .end()
    }
}
