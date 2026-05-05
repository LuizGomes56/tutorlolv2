use super::*;

impl Generator for Phage {
    fn generate(&mut self) -> MayFail {
        self.end()
    }
}
