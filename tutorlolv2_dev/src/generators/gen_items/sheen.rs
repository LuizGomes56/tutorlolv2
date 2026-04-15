use super::*;

impl Generator<ItemData> for Sheen {
    fn generate(mut self: Box<Self>) -> MayFail<ItemData> {
        self.const_min_dmg(BaseAd);
        self.attr(OnhitMax);
        self.damage_type(Physical);
        self.end()
    }
}
