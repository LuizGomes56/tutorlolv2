use super::*;

impl Generator for Hamstringer {
    fn generate(&mut self) -> MayFail {
        // Scour: bleed over 2 seconds on critical strikes, plus 25% of the
        // critical strike instance, which the wiki doesn't write as a ratio
        let bleed = self.render(self.flip(Passive)?)?;
        self.damage_type(Physical)
            .set_min(f![bleed + 0.25 * AttackDamage * CritDamage / 100])
            .end()
    }
}
