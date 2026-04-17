use super::*;

impl Generator<ItemData> for RavenousHydra {
    fn generate(mut self: Box<Self>) -> MayFail<ItemData> {
        let max = self.active(0)?;

        self.melee_min_dmg(max.times(0.5))
            .ranged_min_dmg(max.times(0.25))
            .const_max_dmg(max)
            .attr(Area)
            .damage_type(Physical);

        self.end()
    }
}
