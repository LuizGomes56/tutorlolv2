use super::*;

impl Generator for Heartsteel {
    fn generate(&mut self) -> MayFail {
        let passive = self.passive(1)?;

        let base_dmg = passive.capture_numbers::<i32>()[0];
        let scaling = passive.get_scalings();

        self.const_min_dmg(base_dmg.plus(scaling))
            .damage_type(Physical)
            .attr(OnhitMax)
            .end()
    }
}
