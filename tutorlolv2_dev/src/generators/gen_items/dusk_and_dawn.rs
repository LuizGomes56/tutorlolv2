use super::*;

impl Generator for DuskAndDawn {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
