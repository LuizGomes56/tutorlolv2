use super::*;

impl Generator<ItemData> for IronspikeWhip {
    fn generate(mut self: Box<Self>) -> MayFail<ItemData> {
        self.const_min_dmg(BaseAd).attr(Area).damage_type(Physical);
        self.end()
    }
}
