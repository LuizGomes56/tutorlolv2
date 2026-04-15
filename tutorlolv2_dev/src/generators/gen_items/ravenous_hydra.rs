use super::*;

impl Generator<ItemData> for RavenousHydra {
    fn generate(mut self: Box<Self>) -> MayFail<ItemData> {
        let max_dmg = self.active(0)?;

        self.const_max_dmg(&max_dmg);
        self.melee_min_dmg(max_dmg.half());
        self.ranged_min_dmg(max_dmg.mul(0.25));
        self.attr(Area);
        self.damage_type(Physical);
        self.end()
    }
}
