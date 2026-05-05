use super::*;

impl Generator for RunicCompass {
    fn generate(&mut self) -> MayFail {
        self.min(Active)?.min(Passive)?.end()
    }
}
