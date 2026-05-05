use super::*;

impl Generator for BlackSpear {
    fn generate(&mut self) -> MayFail {
        self.min(Active).end()
    }
}
