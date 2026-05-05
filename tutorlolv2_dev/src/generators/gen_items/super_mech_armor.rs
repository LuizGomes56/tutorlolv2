use super::*;

impl Generator for SuperMechArmor {
    fn generate(&mut self) -> MayFail {
        self.min(Passive)?.end()
    }
}
