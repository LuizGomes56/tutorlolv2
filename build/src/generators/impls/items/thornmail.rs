use super::*;

impl Generator for Thornmail {
    fn generate(&mut self) -> MayFail {
        // Thorns, dealt to attackers
        self.damage_type(Magic).min(Passive)?.end()
    }
}
