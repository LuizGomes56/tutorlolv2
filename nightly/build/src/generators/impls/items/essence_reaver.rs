use super::*;

impl Generator for EssenceReaver {
    fn generate(&mut self) -> MayFail {
        // Spellblade: the wiki writes the critical strike chance part as a
        // progression of 0.5 bonus damage for every 1% critical strike chance
        let damage = self.render_with(self.effect(Passive)?, [1])?;
        self.damage_type(Physical)
            .attr(OnhitMax)
            .set_min(f![damage + 0.5 * CritChance])
            .end()
    }
}
