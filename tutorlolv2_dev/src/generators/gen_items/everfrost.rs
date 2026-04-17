use super::*;

impl Generator<ItemData> for Everfrost {
    fn generate(mut self: Box<Self>) -> MayFail<ItemData> {
        self.const_min_dmg(300.plus(0.85).times(AbilityPower))
            .attr(Area)
            .damage_type(Magic);

        self.end()
    }
}
