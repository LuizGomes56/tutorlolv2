use super::*;

impl Generator for BootsOfSwiftness {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
