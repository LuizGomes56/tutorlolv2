use super::*;

impl Generator for FatedAshes {
    fn generate(&mut self) -> MayFail {
        // Inflame: damage per tick, and the total over 3 seconds
        let total = self.total_formula(Named("Inflame"))?;

        self.damage_type(Magic)
            .min(Named("Inflame"))?
            .set_max(total)
            .end()
    }
}
