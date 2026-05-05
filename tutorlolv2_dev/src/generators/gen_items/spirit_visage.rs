use super::*;

impl Generator for SpiritVisage {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
