use super::*;

impl Generator<ItemData> for BaseTurretReinforcedArmorTurretItem {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
