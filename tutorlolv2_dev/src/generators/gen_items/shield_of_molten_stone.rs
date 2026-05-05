use super::*;

impl Generator for ShieldOfMoltenStone {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
