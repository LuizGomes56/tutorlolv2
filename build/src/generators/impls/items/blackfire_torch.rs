use super::*;

impl Generator for BlackfireTorch {
    fn generate(&mut self) -> MayFail {
        // Baleful Blaze: damage per tick, and the total over 3 seconds
        let total = self.render(self.total(Passive)?)?;
        self.damage_type(Magic)
            .min(Passive)?
            .set_max(total)
            .end()
    }
}
