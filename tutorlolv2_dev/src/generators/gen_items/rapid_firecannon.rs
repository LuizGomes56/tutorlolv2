use super::*;

impl Generator<ItemData> for RapidFirecannon {
    fn generate(mut self: Box<Self>) -> MayFail<ItemData> {
        let damage = self.passive(1)?;
        self.const_min_dmg(damage).attr(OnhitMax).damage_type(Magic);
        self.end()
    }
}
