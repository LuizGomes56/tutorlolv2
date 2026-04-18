use super::*;

impl Generator for EssenceReaver {
    fn generate(&mut self) -> MayFail {
        let passive = self.passive(0)?;
        let (base, rest) = passive
            .split_once(" + ")
            .ok_or("Failed to get base damage")?;
        let scaling = rest.capture_parens(0).unwrap();
        let damage = base.plus(scaling);

        self.const_min_dmg(damage)
            .attr(OnhitMax)
            .damage_type(Physical)
            .end()
    }
}
