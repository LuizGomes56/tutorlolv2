use super::*;

impl Generator<ItemData> for StatikkShiv {
    fn generate(mut self: Box<Self>) -> MayFail<ItemData> {
        let damage = &self.passive(1)?.capture_numbers::<String>()[0];
        self.const_min_dmg(damage);
        self.attr(AreaOnhitMax);
        self.damage_type(Magic);
        self.end()
    }
}
