use super::*;

impl Generator for EndlessHunger {
    fn generate(&mut self) -> MayFail {
        self.end()
    }
}
