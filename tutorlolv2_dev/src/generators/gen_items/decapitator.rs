use super::*;

impl Generator for Decapitator {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
