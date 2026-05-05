use super::*;

impl Generator for Stormrazor {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
