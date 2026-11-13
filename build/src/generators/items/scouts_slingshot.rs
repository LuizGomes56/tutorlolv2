use super::*;

impl Generator for ScoutsSlingshot {
    fn generate(&mut self) -> MayFail {
        // Bullseye
        self.damage_type(Magic).min(Named("Bullseye"))?.end()
    }
}
