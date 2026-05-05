use super::*;

impl Generator for ScorchclawPup {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
