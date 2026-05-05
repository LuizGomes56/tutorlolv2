use super::*;

impl Generator for Morellonomicon {
    fn generate(&mut self) -> MayFail {
        self.min(Passive).end()
    }
}
