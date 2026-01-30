use super::*;

impl Generator<ItemData> for ZephyrArena {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
