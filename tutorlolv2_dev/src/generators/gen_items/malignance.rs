use super::*;

impl Generator<ItemData> for Malignance {
    fn generate(mut self: Box<Self>) -> MayFail<ItemData> {
        let damage_per_tick = 15.plus(0.0125).times(AbilityPower);

        self.const_dmg(&damage_per_tick, 12.times(damage_per_tick.parens()))
            .damage_type(Magic)
            .attr(Area);

        self.end()
    }
}
