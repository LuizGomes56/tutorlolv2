use super::*;

impl Generator<ItemData> for Redemption {
    fn generate(mut self: Box<Self>) -> MayFail<ItemData> {
        let damage = self.active(0)?;
        self.const_min_dmg(damage);
        self.attr(Area);
        self.damage_type(True);
        self.end()
    }
}
