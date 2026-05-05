use super::*;

impl Generator for GuardiansOrb {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
