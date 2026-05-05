use super::*;

impl Generator for Swiftmarch {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
