use super::*;

impl Generator<ItemData> for Everfrost {
    fn generate(mut self: Box<Self>) -> MayFail<ItemData> {
        let damage = 300.plus(0.85).times(AbilityPower);
        self.const_min_dmg(damage);
        self.attr(Area);
        self.damage_type(Magic);
        self.end()
    }
}
