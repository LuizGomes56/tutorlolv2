use super::*;

impl Generator for TitanicHydra {
    fn generate(&mut self) -> MayFail {
        // Cleave bonus damage on-hit, and the empowered Titanic Crescent. The
        // other scalings are the damage to secondary targets
        self.damage_type(Physical)
            .attr(Onhit)
            .min_with(Passive, [0])?
            .max_with(Active, [0])?
            .end()
    }
}
