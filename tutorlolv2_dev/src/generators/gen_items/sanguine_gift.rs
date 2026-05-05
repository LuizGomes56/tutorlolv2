use super::*;

impl Generator for SanguineGift {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
