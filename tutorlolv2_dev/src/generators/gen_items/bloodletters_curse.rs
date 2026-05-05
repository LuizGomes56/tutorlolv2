use super::*;

impl Generator for BloodlettersCurse {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
