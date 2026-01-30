use super::*;

impl Generator<ItemData> for ChainVestArena {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
