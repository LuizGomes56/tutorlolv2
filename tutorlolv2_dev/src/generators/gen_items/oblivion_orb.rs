use super::*;

impl Generator for OblivionOrb {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
