use super::*;

impl Generator for RealityFracture {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
