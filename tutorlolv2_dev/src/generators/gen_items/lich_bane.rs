use super::*;

impl Generator<ItemData> for LichBane {
    fn generate(mut self: Box<Self>) -> MayFail<ItemData> {
        let passive = self.passive(0)?;
        let damage = passive
            .split_once(" + ")
            .ok_or("Unable to remove attack speed property")?
            .1;
        self.const_min_dmg(damage).attr(OnhitMax).damage_type(Magic);
        self.end()
    }
}
