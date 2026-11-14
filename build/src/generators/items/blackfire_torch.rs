use super::*;

impl Generator for BlackfireTorch {
    fn generate(&mut self) -> MayFail {
        // Baleful Blaze: damage per tick, and the total over 3 seconds
        let total = self.total_formula(Named("Baleful Blaze"))?;

        self.damage_type(Magic)
            .min(Named("Baleful Blaze"))?
            .set_max(total)
            .end()
    }
}
