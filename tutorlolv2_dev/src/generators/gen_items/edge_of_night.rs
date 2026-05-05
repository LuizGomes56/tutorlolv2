use super::*;

impl Generator for EdgeOfNight {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
