use super::*;

impl Generator<ItemData> for ReinforcedArmorTurretItem {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
