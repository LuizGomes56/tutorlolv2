use super::*;

impl Generator for TitanicHydra {
    fn generate(&mut self) -> MayFail {
        // Cleave bonus damage on-hit, and the empowered Titanic Crescent. The
        // other scalings are the damage to secondary targets
        self.damage_type(Physical)
            .attr(Onhit)
            .min(Event("Cleave", PrimaryTarget))?
            .max(Event("Titanic Crescent", PrimaryTarget))?
            .end()
    }
}
