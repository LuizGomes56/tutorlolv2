use super::*;

impl Generator for GargoyleStoneplate {
    fn generate(&mut self) -> MayFail {
        self.min(Active)?.end()
    }
}
