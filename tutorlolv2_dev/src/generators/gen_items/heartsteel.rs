use super::*;

impl Generator<ItemData> for Heartsteel {
    fn generate(mut self: Box<Self>) -> MayFail<ItemData> {
        let passive = self.passive(1)?;

        let base_dmg = passive.capture_numbers::<i32>()[0];
        let scaling = passive.get_scalings();

        self.const_min_dmg(format!("{base_dmg} + {scaling}"));
        self.damage_type(Physical);
        self.end()
    }
}
