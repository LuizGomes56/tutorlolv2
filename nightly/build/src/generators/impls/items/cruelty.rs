use super::*;

impl Generator for Cruelty {
    fn generate(&mut self) -> MayFail {
        // Watch Them Fall: comet after immobilizing an enemy champion
        self.damage_type(Magic).min(Passive)?.end()
    }
}
