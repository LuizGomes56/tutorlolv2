use super::*;

impl Generator for VoltaicCyclosword {
    fn generate(&mut self) -> MayFail {
        // Firmament: Energized attack
        self.damage_type(Physical)
            .attr(OnhitMax)
            .min(Named("Firmament"))?
            .end()
    }
}
