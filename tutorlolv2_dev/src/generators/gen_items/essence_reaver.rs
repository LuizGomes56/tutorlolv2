use super::*;

impl Generator<ItemData> for EssenceReaver {
    fn generate(mut self: Box<Self>) -> MayFail<ItemData> {
        let passive = self.passive(0)?;
        let (base, rest) = passive
            .split_once(" + ")
            .ok_or("Failed to get base damage")?;
        let scaling = rest.capture_parens(0).unwrap();
        let damage = base.plus(scaling);

        self.const_min_dmg(damage)
            .attr(OnhitMax)
            .damage_type(Physical);

        self.end()
    }
}
