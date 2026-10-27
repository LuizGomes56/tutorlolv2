use super::*;

impl Generator for PyromancersCloak {
    fn generate(&mut self) -> MayFail {
        // Spark: burn per tick, and the total over 3 seconds
        let tick = self.render(self.flip(Passive)?)?;
        self.damage_type(Magic)
            .set_min(tick)
            .max(Passive)?
            .end()
    }
}
