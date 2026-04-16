use super::*;

impl Generator<ItemData> for Stridebreaker {
    fn generate(mut self: Box<Self>) -> MayFail<ItemData> {
        let passive = self.passive(0)?;
        let active = self.active(0)?;

        let max_dmg = active
            .split_once(" + ")
            .ok_or("Failed to get active damage")?
            .0;

        let (melee, ranged) = passive
            .split_once(" + ")
            .ok_or("Failed to get melee/ranged passive damage")?;

        self.melee_min_dmg(melee);
        self.ranged_min_dmg(ranged);
        self.const_max_dmg(max_dmg);
        self.attr(AreaOnhit);
        self.damage_type(Physical);
        self.end()
    }
}
