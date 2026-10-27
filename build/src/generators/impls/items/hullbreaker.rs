use super::*;

impl Generator for Hullbreaker {
    fn generate(&mut self) -> MayFail {
        // Empowered attack against champions at maximum stacks. The other
        // scalings are its damage against structures
        self.damage_type(Physical)
            .attr(OnhitMax)
            .min_with(Passive, [0, 2])?
            .end()
    }
}
