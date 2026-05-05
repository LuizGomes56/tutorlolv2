use super::*;

impl Generator for RavenousHydra {
    fn generate(&mut self) -> MayFail {
        self.min(Active)?.damage_type(Physical).end()
    }
}
