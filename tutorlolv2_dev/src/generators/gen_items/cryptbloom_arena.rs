use super::*;

impl Generator<ItemData> for CryptbloomArena {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
