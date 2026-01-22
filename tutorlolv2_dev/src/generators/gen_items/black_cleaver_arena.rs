use super::*;

impl Generator<ItemData> for BlackCleaverArena {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
