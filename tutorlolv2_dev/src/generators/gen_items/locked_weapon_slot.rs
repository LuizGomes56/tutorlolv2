use super::*;

impl Generator<ItemData> for LockedWeaponSlot {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
