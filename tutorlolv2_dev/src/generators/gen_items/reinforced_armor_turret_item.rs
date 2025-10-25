use super::*;

impl Generator<ItemData> for ReinforcedArmorTurretItem {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
