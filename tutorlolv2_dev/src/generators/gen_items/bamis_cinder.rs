use super::*;

impl Generator<ItemData> for BamisCinder {
    fn generate(mut self: Box<Self>) -> MayFail<ItemData> {
        let damage = self.passive(0)?;
        self.const_min_dmg(damage).damage_type(Magic).attr(Area);
        self.end()
    }
}
