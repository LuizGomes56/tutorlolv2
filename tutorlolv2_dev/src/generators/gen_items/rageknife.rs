use super::*;

impl Generator<ItemData> for Rageknife {
    fn generate(mut self: Box<Self>) -> MayFail<ItemData> {
        self.const_min_dmg("20");
        self.attr(Onhit);
        self.damage_type(Magic);
        self.end()
    }
}
