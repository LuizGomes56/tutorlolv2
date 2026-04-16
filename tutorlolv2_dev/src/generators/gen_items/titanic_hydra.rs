use super::*;

impl Generator<ItemData> for TitanicHydra {
    fn generate(mut self: Box<Self>) -> MayFail<ItemData> {
        let [melee, ranged] = self.capture(Active(0), |v| format!("{v} * {MaxHealth}"))?;

        self.melee_max_dmg(&melee);
        self.ranged_max_dmg(&ranged);
        self.melee_min_dmg(melee.mul(0.25));
        self.ranged_min_dmg(ranged.mul(0.25));
        self.attr(AreaOnhit);
        self.damage_type(Physical);
        self.end()
    }
}
