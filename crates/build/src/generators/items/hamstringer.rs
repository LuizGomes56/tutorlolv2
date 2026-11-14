use super::*;

impl Generator for Hamstringer {
    fn generate(&mut self) -> MayFail {
        // Scour: bleed over 2 seconds on critical strikes, plus 25% of the
        // critical strike instance, which the wiki doesn't write as a ratio
        let bleed = self.alternate_formula(Event("Scour", CriticalBleed))?;
        let ratio = self.critical_instance_ratio(Named("Scour"))?;

        self.damage_type(Physical)
            .set_min(f![bleed + ratio * AttackDamage * CritDamage / 100])
            .end()
    }
}
