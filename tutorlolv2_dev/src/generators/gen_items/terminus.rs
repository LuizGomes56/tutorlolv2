use super::*;

impl Generator for Terminus {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
