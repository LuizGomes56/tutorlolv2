use super::*;

impl Generator for HextechRocketbelt {
    fn generate(&mut self) -> MayFail {
        self.damage_type(Magic).min(Named("Supersonic"))?.end()
    }
}
