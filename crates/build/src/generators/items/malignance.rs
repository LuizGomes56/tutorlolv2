use super::*;

impl Generator for Malignance {
    fn generate(&mut self) -> MayFail {
        // Hatefog: damage per tick, and the total over 3 seconds
        let total = self.alternate_formula(Event("Hatefog", FogDamage))?;

        self.damage_type(Magic)
            .min(Event("Hatefog", FogDamage))?
            .set_max(total)
            .end()
    }
}
