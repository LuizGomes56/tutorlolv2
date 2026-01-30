use super::*;

impl Generator<ItemData> for UnendingDespairArena {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
