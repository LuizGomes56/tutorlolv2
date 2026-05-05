use super::*;

impl Generator for DarkSeal {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
