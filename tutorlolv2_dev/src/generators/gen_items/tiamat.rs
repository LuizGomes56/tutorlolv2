use super::*;

impl Generator<ItemData> for Tiamat {
    fn generate(mut self: Box<Self>) -> MayFail<ItemData> {
        let max = self.active(0)?;
        let passive = self.passive(0)?;
        let (melee, ranged) = passive
            .split_once(" + ")
            .ok_or("Failed to get melee and ranged passive damage")?;

        self.const_max_dmg(max)
            .melee_min_dmg(melee)
            .ranged_min_dmg(ranged)
            .attr(Area)
            .damage_type(Physical);

        self.end()
    }
}
