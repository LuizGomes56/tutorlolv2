use super::*;

impl Generator for ZekesConvergence {
    fn generate(&mut self) -> MayFail {
        // Frostfire Tempest: damage per tick, and the total over 5 seconds
        let total = self.render(self.flip(Passive2)?)?;
        self.damage_type(Magic)
            .min(Passive2)?
            .set_max(total)
            .end()
    }
}
