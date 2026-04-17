use super::*;

impl Generator<ItemData> for RavenousHydra {
    fn generate(mut self: Box<Self>) -> MayFail<ItemData> {
        let max_dmg = self.active(0)?;

        self.melee_min_dmg(max_dmg.times(0.5));
        self.ranged_min_dmg(max_dmg.times(0.25));
        self.const_max_dmg(max_dmg);
        self.attr(Area);
        self.damage_type(Physical);
        self.end()
    }
}
