use super::*;

impl Generator<ItemData> for HollowRadianceArena {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
