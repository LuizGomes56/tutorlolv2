use super::*;

impl Generator for ZazZaksRealmspike {
    fn generate(&mut self) -> MayFail {
        // Explosion after dealing ability damage
        self.damage_type(Magic).min(Passive)?.end()
    }
}
