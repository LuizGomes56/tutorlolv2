use super::*;

impl Generator<ItemData> for WhisperingCircletArena {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
