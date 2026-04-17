use super::*;

impl Generator<ItemData> for HextechRocketbelt {
    fn generate(mut self: Box<Self>) -> MayFail<ItemData> {
        let damage = self.active(0)?;
        self.const_min_dmg(damage).attr(Area).damage_type(Magic);
        self.end()
    }
}
