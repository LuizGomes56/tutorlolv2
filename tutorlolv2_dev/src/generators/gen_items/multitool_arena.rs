use super::*;

impl Generator<ItemData> for MultitoolArena {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
