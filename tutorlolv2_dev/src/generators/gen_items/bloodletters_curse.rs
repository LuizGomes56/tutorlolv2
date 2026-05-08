use super::*;

impl Generator for BloodlettersCurse {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
