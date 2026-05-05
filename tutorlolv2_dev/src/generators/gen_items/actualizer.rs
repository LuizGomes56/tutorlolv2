use super::*;

impl Generator for Actualizer {
    fn generate(&mut self) -> MayFail {
        self.min(Active).end()
    }
}
