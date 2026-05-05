use super::*;

impl Generator for KnightsVow {
    fn generate(&mut self) -> MayFail {
        self.min(Active)?.min(Passive)?.end()
    }
}
