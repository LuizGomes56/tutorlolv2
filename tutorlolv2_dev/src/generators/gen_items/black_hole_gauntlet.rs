use super::*;

impl Generator for BlackHoleGauntlet {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
