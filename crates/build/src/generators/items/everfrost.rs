use super::*;

impl Generator for Everfrost {
    fn generate(&mut self) -> MayFail {
        // Glaciate
        self.damage_type(Magic).min(Named("Glaciate"))?.end()
    }
}
