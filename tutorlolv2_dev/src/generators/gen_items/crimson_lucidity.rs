use super::*;

impl Generator for CrimsonLucidity {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
