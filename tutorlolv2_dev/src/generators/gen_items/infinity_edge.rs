use super::*;

impl Generator<ItemData> for InfinityEdge {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
