use super::*;

impl Generator for Stormsurge {
    fn generate(&mut self) -> MayFail {
        // Squall
        self.damage_type(Magic).min(Passive2)?.end()
    }
}
