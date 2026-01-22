use super::*;

impl Generator<ItemData> for RuneglaiveArena {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
