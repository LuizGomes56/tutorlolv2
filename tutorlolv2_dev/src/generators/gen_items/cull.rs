use super::*;

impl Generator for Cull {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
