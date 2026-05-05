use super::*;

impl Generator for GuardiansHorn {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
