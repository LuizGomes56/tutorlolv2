use super::*;

impl Generator for TitanicHydra {
    fn generate(&mut self) -> MayFail {
        let [melee, ranged] = self.capture(Active(0), |v| v.times(MaxHealth))?;

        self.melee_max_dmg(&melee)
            .ranged_max_dmg(&ranged)
            .melee_min_dmg(melee.times(0.25))
            .ranged_min_dmg(ranged.times(0.25))
            .attr(AreaOnhit)
            .damage_type(Physical)
            .end()
    }
}
