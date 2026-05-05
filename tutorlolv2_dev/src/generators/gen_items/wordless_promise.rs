use super::*;

impl Generator for WordlessPromise {
    fn generate(&mut self) -> MayFail {
        self.min(Active)?.min(Passive)?.end()
    }
}
