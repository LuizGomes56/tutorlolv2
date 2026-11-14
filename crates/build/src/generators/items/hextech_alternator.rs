use super::*;

impl Generator for HextechAlternator {
    fn generate(&mut self) -> MayFail {
        self.damage_type(Magic).min(Named("Revved"))?.end()
    }
}
