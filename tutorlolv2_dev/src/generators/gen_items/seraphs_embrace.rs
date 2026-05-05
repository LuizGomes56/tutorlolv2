use super::*;

impl Generator for SeraphsEmbrace {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
