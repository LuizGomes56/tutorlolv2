use super::*;

impl Generator<ItemData> for RedemptionArena {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
