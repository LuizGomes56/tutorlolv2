use super::*;

impl Generator for DeadMansPlate {
    fn generate(&mut self) -> MayFail {
        self.const_min_dmg(BaseAd)
            .attr(OnhitMax)
            .damage_type(Physical)
            .end()
    }
}
