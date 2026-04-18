use super::*;

impl Generator for Muramana {
    fn generate(&mut self) -> MayFail {
        let scaling = self.passive(0)?.capture_numbers::<f64>()[0];
        let ctx = |v| MaxMana.times(v / 100.0);
        let [melee, ranged] = [4.0, 3.0].map(ctx);

        self.const_min_dmg(ctx(scaling))
            .melee_max_dmg(melee)
            .ranged_max_dmg(ranged)
            .attr(Onhit)
            .damage_type(Physical)
            .end()
    }
}
