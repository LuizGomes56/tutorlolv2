use super::*;

impl Generator for HextechAlternator {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
