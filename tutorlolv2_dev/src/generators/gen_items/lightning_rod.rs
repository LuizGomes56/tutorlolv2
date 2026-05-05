use super::*;

impl Generator for LightningRod {
    fn generate(&mut self) -> MayFail {
        self.end()
    }
}
