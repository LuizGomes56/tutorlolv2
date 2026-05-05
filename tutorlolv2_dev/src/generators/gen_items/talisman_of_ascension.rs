use super::*;

impl Generator for TalismanOfAscension {
    fn generate(&mut self) -> MayFail {
        self.min(Active).end()
    }
}
