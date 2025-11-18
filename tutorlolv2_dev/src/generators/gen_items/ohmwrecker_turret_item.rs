use super::*;

impl Generator<ItemData> for OhmwreckerTurretItem {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
