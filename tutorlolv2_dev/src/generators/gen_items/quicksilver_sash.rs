use super::*;

impl Generator for QuicksilverSash {
    fn generate(&mut self) -> MayFail {
        self.min(Active)?.end()
    }
}
