use super::*;

impl Generator for SwordOfBlossomingDawn {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
