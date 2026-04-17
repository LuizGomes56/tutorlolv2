use super::*;

impl Generator<ItemData> for DeadMansPlate {
    fn generate(mut self: Box<Self>) -> MayFail<ItemData> {
        self.const_min_dmg(BaseAd)
            .attr(OnhitMax)
            .damage_type(Physical);

        self.end()
    }
}
