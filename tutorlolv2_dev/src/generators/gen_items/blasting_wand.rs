use super::*;

impl Generator for BlastingWand {
    fn generate(&mut self) -> MayFail {
        self.end()
    }
}
