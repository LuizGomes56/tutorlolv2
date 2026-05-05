use super::*;

impl Generator for RejuvenationBead {
    fn generate(&mut self) -> MayFail {
        self.end()
    }
}
