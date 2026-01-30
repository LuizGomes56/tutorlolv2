use super::*;

impl Generator<ItemData> for MuramanaArena {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
