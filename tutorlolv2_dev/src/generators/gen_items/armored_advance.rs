use super::*;

impl Generator for ArmoredAdvance {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
