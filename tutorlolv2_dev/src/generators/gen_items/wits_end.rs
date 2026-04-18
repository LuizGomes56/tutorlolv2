use super::*;

impl Generator for WitsEnd {
    fn generate(&mut self) -> MayFail {
        let damage = self.passive(0)?;
        self.const_min_dmg(damage)
            .attr(Onhit)
            .damage_type(Magic)
            .end()
    }
}
