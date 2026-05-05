use super::*;

impl Generator for VoltaicCyclosword {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
