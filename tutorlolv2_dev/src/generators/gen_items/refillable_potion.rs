use super::*;

impl Generator for RefillablePotion {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
