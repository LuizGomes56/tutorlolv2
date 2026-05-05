use super::*;

impl Generator for Hubris {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
