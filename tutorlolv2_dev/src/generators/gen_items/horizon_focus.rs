use super::*;

impl Generator for HorizonFocus {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.damage_type(True).end()
    }
}
