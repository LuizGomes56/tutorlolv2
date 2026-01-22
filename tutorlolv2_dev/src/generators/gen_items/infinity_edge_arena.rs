use super::*;

impl Generator<ItemData> for InfinityEdgeArena {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
