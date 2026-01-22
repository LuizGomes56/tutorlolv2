use super::*;

impl Generator<ItemData> for HeartsteelArena {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
