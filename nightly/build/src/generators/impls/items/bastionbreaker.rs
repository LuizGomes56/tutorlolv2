use super::*;

impl Generator for Bastionbreaker {
    fn generate(&mut self) -> MayFail {
        // Shaped Charge: bonus true damage on the next ability hit
        self.damage_type(True).min(Passive)?.end()
    }
}
