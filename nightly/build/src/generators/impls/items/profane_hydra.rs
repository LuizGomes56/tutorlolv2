use super::*;

impl Generator for ProfaneHydra {
    fn generate(&mut self) -> MayFail {
        // Heretical Cleave
        self.damage_type(Physical).min(Active)?.end()
    }
}
