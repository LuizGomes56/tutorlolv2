use super::*;

impl Generator<ItemData> for LiandrysTorment {
    fn generate(mut self: Box<Self>) -> MayFail<ItemData> {
        let passive = self.passive(0)?;
        let (min, max) = passive
            .split_once(" + ")
            .ok_or("Failed to get passive damage")?;

        self.const_min_dmg(min);
        self.const_max_dmg(max);
        self.damage_type(Magic);
        self.end()
    }
}
