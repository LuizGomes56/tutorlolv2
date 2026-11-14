use super::*;

impl Generator for WitsEnd {
    fn generate(&mut self) -> MayFail {
        // Fray
        self.damage_type(Magic)
            .attr(Onhit)
            .min(Named("Fray"))?
            .end()
    }
}
