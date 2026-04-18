use super::*;

impl Generator for IronspikeWhip {
    fn generate(&mut self) -> MayFail {
        self.const_min_dmg(BaseAd).attr(Area).damage_type(Physical).end()
    }
}
