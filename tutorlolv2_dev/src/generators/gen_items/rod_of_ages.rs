use super::*;

impl Generator for RodOfAges {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
