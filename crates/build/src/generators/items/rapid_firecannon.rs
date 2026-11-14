use super::*;

impl Generator for RapidFirecannon {
    fn generate(&mut self) -> MayFail {
        // Sharpshooter: Energized attack
        self.damage_type(Magic)
            .attr(OnhitMax)
            .min(Named("Sharpshooter"))?
            .end()
    }
}
