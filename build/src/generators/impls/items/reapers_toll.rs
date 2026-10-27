use super::*;

impl Generator for ReapersToll {
    fn generate(&mut self) -> MayFail {
        // Reap: Stacks = Reap stacks on the target. The other scalings are the
        // health threshold it inflicts
        self.damage_type(True)
            .attr(Onhit)
            .min_with(Passive, [0])?
            .end()
    }
}
