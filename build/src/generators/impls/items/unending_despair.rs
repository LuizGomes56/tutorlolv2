use super::*;

impl Generator for UnendingDespair {
    fn generate(&mut self) -> MayFail {
        // Anguish
        self.damage_type(Magic).min(Passive)?.end()
    }
}
