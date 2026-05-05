use super::*;

impl Generator for Cryptbloom {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
