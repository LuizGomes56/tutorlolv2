use super::*;

impl Generator<ItemData> for Rageknife {
    fn generate(mut self: Box<Self>) -> MayFail<ItemData> {
        self.const_min_dmg(20).attr(Onhit).damage_type(Magic);
        self.end()
    }
}
