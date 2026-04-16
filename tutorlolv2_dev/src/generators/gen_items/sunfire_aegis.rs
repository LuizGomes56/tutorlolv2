use super::*;

impl Generator<ItemData> for SunfireAegis {
    fn generate(mut self: Box<Self>) -> MayFail<ItemData> {
        let damage = self.passive(0)?;
        self.const_min_dmg(damage);
        self.attr(Area);
        self.damage_type(Magic);
        self.end()
    }
}
