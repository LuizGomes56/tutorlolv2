use super::*;

impl Generator<ItemData> for RecurveBow {
    fn generate(mut self: Box<Self>) -> MayFail<ItemData> {
        let damage = self.passive(0)?;
        self.const_min_dmg(damage).attr(Onhit).damage_type(Physical);
        self.end()
    }
}
