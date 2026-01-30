use super::*;

impl Generator<ItemData> for PillorySwipe {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
