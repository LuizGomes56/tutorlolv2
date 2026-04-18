use super::*;

impl Generator for Sheen {
    fn generate(&mut self) -> MayFail {
        let damage = self.passive(0)?;
        self.const_min_dmg(damage)
            .attr(OnhitMax)
            .damage_type(Physical).end()
    }
}
