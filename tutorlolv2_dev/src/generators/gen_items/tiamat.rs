use super::*;

impl Generator<ItemData> for Tiamat {
    fn generate(mut self: Box<Self>) -> MayFail<ItemData> {
        let max_dmg = self.active(0)?;
        self.const_max_dmg(max_dmg);

        let passive = self.passive(0)?;
        let (melee, ranged) = passive
            .split_once(" + ")
            .ok_or("Failed to get melee and ranged passive damage")?;

        self.melee_min_dmg(melee);
        self.ranged_min_dmg(ranged);
        self.attr(Area);
        self.damage_type(Physical);
        self.end()
    }
}
