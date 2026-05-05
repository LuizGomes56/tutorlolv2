use super::*;

impl Generator for SuperMechPowerField {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
