use super::*;

impl Generator for UmbralGlaive {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
