use super::*;

impl Generator for LiandrysTorment {
    fn generate(&mut self) -> MayFail {
        // Torment: damage per tick, and the total over 3 seconds
        let total = self.render(self.flip(Passive)?)?;
        self.damage_type(Magic)
            .min(Passive)?
            .set_max(total)
            .end()
    }
}
