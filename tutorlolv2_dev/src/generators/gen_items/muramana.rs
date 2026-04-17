use super::*;

impl Generator<ItemData> for Muramana {
    fn generate(mut self: Box<Self>) -> MayFail<ItemData> {
        let scaling = self.passive(0)?.capture_numbers::<f64>()[0];
        let ctx = |v| MaxMana.times(v / 100.0);
        let [melee, ranged] = [4.0, 3.0].map(ctx);

        self.const_min_dmg(ctx(scaling));
        self.melee_max_dmg(melee);
        self.ranged_max_dmg(ranged);
        self.attr(Onhit);
        self.damage_type(Physical);
        self.end()
    }
}
