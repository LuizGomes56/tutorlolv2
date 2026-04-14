use super::*;

impl Generator<ItemData> for GuinsoosRageblade {
    fn generate(mut self: Box<Self>) -> MayFail<ItemData> {
        let damage = self.passive(0)?;
        self.const_min_dmg(damage);
        self.attr(Onhit);
        self.damage_type(Magic);
        self.end()
    }
}
