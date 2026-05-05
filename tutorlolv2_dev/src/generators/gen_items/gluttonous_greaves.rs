use super::*;

impl Generator for GluttonousGreaves {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
