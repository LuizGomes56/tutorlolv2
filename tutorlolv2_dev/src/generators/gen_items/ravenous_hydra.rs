use super::*;

impl Generator<ItemData> for RavenousHydra {
    fn generate(mut self: Box<Self>) -> MayFail<ItemData> {
        let mul = self
            .active(0)?
            .capture_numbers::<f64>()
            .get(0)
            .ok_or("Failed to get multiplier active::numbers::[0]")?
            / 100.0;

        let max_dmg = format!("{mul} * {AttackDamage}");
        self.const_max_dmg(&max_dmg);
        self.melee_min_dmg(max_dmg.half());
        self.ranged_min_dmg(max_dmg.half().half());
        self.attr(OnhitMax);
        self.damage_type(Physical);
        self.end()
    }
}
