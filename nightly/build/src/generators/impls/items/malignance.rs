use super::*;

impl Generator for Malignance {
    fn generate(&mut self) -> MayFail {
        // Hatefog: damage per tick, and the total over 3 seconds
        let total = self.render(self.flip(Passive2)?)?;
        self.damage_type(Magic)
            .min(Passive2)?
            .set_max(total)
            .end()
    }
}
