use super::*;

impl Generator for Stridebreaker {
    fn generate(&mut self) -> MayFail {
        let passive = self.passive(0)?;
        let active = self.active(0)?;

        let max = active
            .split_once(" + ")
            .ok_or("Failed to get active damage")?
            .0;

        let (melee, ranged) = passive
            .split_once(" + ")
            .ok_or("Failed to get melee/ranged passive damage")?;

        self.melee_min_dmg(melee)
            .ranged_min_dmg(ranged)
            .const_max_dmg(max)
            .attr(AreaOnhit)
            .damage_type(Physical)
            .end()
    }
}
