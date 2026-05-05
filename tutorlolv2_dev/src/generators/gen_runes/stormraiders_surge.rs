use super::*;

impl Generator for StormraidersSurge {
    fn generate(&mut self) -> MayFail {
        self.min(0) /* Passive */
            .end()
    }
}
