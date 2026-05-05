use super::*;

impl Generator for RiteOfRuin {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
