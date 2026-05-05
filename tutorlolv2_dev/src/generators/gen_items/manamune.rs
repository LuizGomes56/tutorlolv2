use super::*;

impl Generator for Manamune {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
