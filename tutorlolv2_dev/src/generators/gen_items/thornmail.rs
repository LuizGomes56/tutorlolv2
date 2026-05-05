use super::*;

impl Generator for Thornmail {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
