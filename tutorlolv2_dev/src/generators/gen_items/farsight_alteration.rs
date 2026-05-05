use super::*;

impl Generator for FarsightAlteration {
    fn generate(&mut self) -> MayFail {
        self.min(Active)?.end()
    }
}
