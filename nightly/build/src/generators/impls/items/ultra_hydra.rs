use super::*;

impl Generator for UltraHydra {
    fn generate(&mut self) -> MayFail {
        // Ultra Cleave: bonus damage to the attack target of the empowered attack
        self.damage_type(Physical)
            .attr(OnhitMax)
            .min_with(Active, [1])?
            .end()
    }
}
