use super::*;

impl Generator for Fimbulwinter {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
