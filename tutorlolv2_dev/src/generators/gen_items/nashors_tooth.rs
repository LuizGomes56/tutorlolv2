use super::*;

// #![stable]

impl Generator<ItemData> for NashorsTooth {
    fn generate(mut self: Box<Self>) -> MayFail<ItemData> {
        let damage = self.passive(0)?;
        self.const_min_dmg(damage);
        self.attr(Onhit);
        self.damage_type(Magic);
        self.end()
    }
}
