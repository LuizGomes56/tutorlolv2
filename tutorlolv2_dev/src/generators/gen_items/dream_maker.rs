use super::*;

impl Generator for DreamMaker {
    fn generate(&mut self) -> MayFail {
        self.min(Active)?.end()
    }
}
