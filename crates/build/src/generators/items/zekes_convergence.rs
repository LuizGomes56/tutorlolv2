use super::*;

impl Generator for ZekesConvergence {
    fn generate(&mut self) -> MayFail {
        // Frostfire Tempest: damage per tick, and the total over 5 seconds
        let total = self.alternate_formula(Named("Frostfire Tempest"))?;

        self.damage_type(Magic)
            .min(Named("Frostfire Tempest"))?
            .set_max(total)
            .end()
    }
}
