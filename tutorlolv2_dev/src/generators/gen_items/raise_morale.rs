use super::*;

impl Generator for RaiseMorale {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
