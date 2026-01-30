use super::*;

impl Generator<ItemData> for MalignanceArena {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
