use super::*;

impl Generator for Lifeline {
    fn generate(&mut self) -> MayFail {
        self.min(Active).end()
    }
}
