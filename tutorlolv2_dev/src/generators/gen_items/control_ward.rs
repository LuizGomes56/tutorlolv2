use super::*;

impl Generator for ControlWard {
    fn generate(&mut self) -> MayFail {
        self.end()
    }
}
