use super::*;

impl Generator<ItemData> for DawncoreArena {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
