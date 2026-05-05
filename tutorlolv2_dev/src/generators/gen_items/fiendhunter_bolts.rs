use super::*;

impl Generator for FiendhunterBolts {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
