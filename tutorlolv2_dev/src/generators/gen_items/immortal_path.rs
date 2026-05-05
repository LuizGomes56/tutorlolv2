use super::*;

impl Generator for ImmortalPath {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
