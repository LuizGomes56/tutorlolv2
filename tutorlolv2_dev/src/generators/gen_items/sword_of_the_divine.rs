use super::*;

impl Generator for SwordOfTheDivine {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
