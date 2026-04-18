use super::*;

impl Generator for Everfrost {
    fn generate(&mut self) -> MayFail {
        self.const_min_dmg(300.plus(0.85).times(AbilityPower))
            .attr(Area)
            .damage_type(Magic)
            .end()
    }
}
