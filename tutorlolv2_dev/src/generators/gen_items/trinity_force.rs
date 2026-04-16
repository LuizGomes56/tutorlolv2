use super::*;

impl Generator<ItemData> for TrinityForce {
    fn generate(mut self: Box<Self>) -> MayFail<ItemData> {
        let damage = self.passive(0)?;
        self.const_min_dmg(damage);
        self.attr(OnhitMax);
        self.damage_type(Physical);
        self.end()
    }
}
