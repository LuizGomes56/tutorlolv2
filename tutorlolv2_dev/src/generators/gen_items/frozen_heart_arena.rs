use super::*;

impl Generator<ItemData> for FrozenHeartArena {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
