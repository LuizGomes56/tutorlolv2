use super::*;

impl Generator for BrambleVest {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
