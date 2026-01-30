use super::*;

impl Generator<ItemData> for RiftmakerArena {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
