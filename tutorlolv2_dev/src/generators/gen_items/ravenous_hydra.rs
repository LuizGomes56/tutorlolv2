use super::*;

impl Generator for RavenousHydra {
    fn generate(&mut self) -> MayFail {
        self.damage_type(Physical).min(Active)?.end()
    }
}
