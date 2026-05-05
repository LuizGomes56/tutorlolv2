use super::*;

impl Generator for SecondWind {
    fn generate(&mut self) -> MayFail {
        self.min(0) /* Passive */
            .end()
    }
}
