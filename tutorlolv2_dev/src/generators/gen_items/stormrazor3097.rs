use super::*;

impl Generator<ItemData> for Stormrazor3097 {
    fn generate(mut self: Box<Self>) -> MayFail<ItemData> {
        let passive = self.passive(1)?;
        let damage = passive
            .split_once(" + ")
            .ok_or("Failed to ignore move speed scaling")?
            .0;
        self.const_min_dmg(damage).attr(OnhitMax).damage_type(Magic);
        self.end()
    }
}
