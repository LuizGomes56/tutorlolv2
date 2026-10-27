use super::*;

impl Generator for Heartsteel {
    fn generate(&mut self) -> MayFail {
        // Colossal Consumption: empowered attack against a target with 3 stacks
        self.damage_type(Physical)
            .attr(OnhitMax)
            .min(Passive)?
            .end()
    }
}
