use super::*;

impl Generator for NeedlesslyLargeRod {
    fn generate(&mut self) -> MayFail {
        self.end()
    }
}
