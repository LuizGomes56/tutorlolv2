use super::*;

impl Generator<ItemData> for RunaansHurricane {
    fn generate(mut self: Box<Self>) -> MayFail<ItemData> {
        let damage = self.passive(0)?;
        self.ranged_min_dmg(damage);
        self.attr(Area);
        self.damage_type(Physical);
        self.end()
    }
}
