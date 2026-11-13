use super::*;

impl Generator for LiandrysTorment {
    fn generate(&mut self) -> MayFail {
        // Torment: damage per tick, and the total over 3 seconds
        let total = self.alternate_formula(Named("Torment"))?;

        self.damage_type(Magic)
            .min(Named("Torment"))?
            .set_max(total)
            .end()
    }
}
