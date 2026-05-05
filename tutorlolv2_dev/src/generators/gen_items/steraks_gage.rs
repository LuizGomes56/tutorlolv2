use super::*;

impl Generator for SteraksGage {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
