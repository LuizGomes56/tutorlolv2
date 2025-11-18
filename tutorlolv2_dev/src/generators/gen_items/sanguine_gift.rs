use super::*;

impl Generator<ItemData> for SanguineGift {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
