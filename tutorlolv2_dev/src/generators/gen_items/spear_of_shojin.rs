use super::*;

impl Generator for SpearOfShojin {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
