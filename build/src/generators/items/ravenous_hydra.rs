use super::*;

impl Generator for RavenousHydra {
    fn generate(&mut self) -> MayFail {
        // Ravenous Crescent
        self.damage_type(Physical)
            .min(Named("Ravenous Crescent"))?
            .end()
    }
}
