use super::*;

impl Generator for StatikkShiv {
    fn generate(&mut self) -> MayFail {
        let damage = &self.passive(1)?.capture_numbers::<String>()[0];

        self.const_min_dmg(damage)
            .attr(AreaOnhitMax)
            .damage_type(Magic)
            .end()
    }
}
