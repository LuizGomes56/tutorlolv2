use super::*;

impl Generator for Malignance {
    fn generate(&mut self) -> MayFail {
        let damage_per_tick = 15.plus(0.0125).times(AbilityPower);

        self.const_dmg(&damage_per_tick, 12.times(damage_per_tick.parens()))
            .damage_type(Magic)
            .attr(Area)
            .end()
    }
}
