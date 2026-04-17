use super::*;

impl Generator<ItemData> for Malignance {
    fn generate(mut self: Box<Self>) -> MayFail<ItemData> {
        let damage_per_tick = 15.plus(0.0125).times(AbilityPower);

        self.const_min_dmg(&damage_per_tick);
        self.const_max_dmg(12.times(damage_per_tick.parens()));
        self.damage_type(Magic);
        self.attr(Area);
        self.end()
    }
}
