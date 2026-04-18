use super::*;

impl Generator for BamisCinder {
    fn generate(&mut self) -> MayFail {
        let damage = self.passive(0)?;
        self.const_min_dmg(damage)
            .damage_type(Magic)
            .attr(Area)
            .end()
    }
}
