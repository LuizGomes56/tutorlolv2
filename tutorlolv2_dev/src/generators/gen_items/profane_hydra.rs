use super::*;

impl Generator for ProfaneHydra {
    fn generate(&mut self) -> MayFail {
        self.min(Active)?.damage_type(Physical).end()
    }
}
