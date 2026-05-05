use super::*;

impl Generator for NullMagicMantle {
    fn generate(&mut self) -> MayFail {
        self.end()
    }
}
