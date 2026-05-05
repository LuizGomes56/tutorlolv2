use super::*;

impl Generator for LastStand {
    fn generate(&mut self) -> MayFail {
        self.min(0) /* Passive */
            .end()
    }
}
