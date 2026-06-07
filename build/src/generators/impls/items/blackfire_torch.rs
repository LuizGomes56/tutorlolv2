use super::*;

impl Generator for BlackfireTorch {
    #[warn(unstable_features)]
    fn generate(&mut self) -> MayFail {
        let scaling = self.scaling(Passive, [1])?;
        let max_dmg = self.base(Passive)?[0].times(scaling);
        let min_dmg = max_dmg.div(6);
        self.damage_type(Magic)
            .asgn_min(min_dmg)
            .asgn_max(max_dmg)
            .end()
    }
}
