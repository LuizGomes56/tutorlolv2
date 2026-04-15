use super::*;

impl Generator<ItemData> for IronspikeWhip {
    fn generate(mut self: Box<Self>) -> MayFail<ItemData> {
        self.const_min_dmg(BaseAd);
        self.attr(Area);
        self.damage_type(Physical);
        self.end()
    }
}
