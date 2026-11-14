use super::*;

impl Generator for EssenceReaver {
    fn generate(&mut self) -> MayFail {
        // Spellblade: the wiki writes the critical strike chance part as a
        // progression of 0.5 bonus damage for every 1% critical strike chance
        let damage = self.formula(Named("Spellblade"))?;

        self.damage_type(Physical)
            .attr(OnhitMax)
            .set_min(damage)
            .end()
    }
}
