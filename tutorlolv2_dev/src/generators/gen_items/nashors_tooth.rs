use super::*;

impl Generator for NashorsTooth {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
