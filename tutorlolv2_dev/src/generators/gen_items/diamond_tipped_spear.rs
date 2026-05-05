use super::*;

impl Generator for DiamondTippedSpear {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
