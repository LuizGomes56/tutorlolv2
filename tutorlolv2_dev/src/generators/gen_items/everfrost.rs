use super::*;

impl Generator<ItemData> for Everfrost {
    fn generate(mut self: Box<Self>) -> MayFail<ItemData> {
        let damage = format!("300 + 0.85 * {AbilityPower}");
        self.const_min_dmg(damage);
        self.attr(Area);
        self.damage_type(Magic);
        self.end()
    }
}
