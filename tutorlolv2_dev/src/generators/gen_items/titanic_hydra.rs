use super::*;

impl Generator for TitanicHydra {
    fn generate(&mut self) -> MayFail {
        self.damage_type(Physical).end()
    }
}
