use super::*;

impl Generator for GuinsoosRageblade {
    fn generate(&mut self) -> MayFail {
        // Wrath
        self.damage_type(Magic)
            .attr(Onhit)
            .min(Named("Wrath"))?
            .end()
    }
}
