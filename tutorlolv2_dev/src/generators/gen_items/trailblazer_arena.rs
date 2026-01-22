use super::*;

impl Generator<ItemData> for TrailblazerArena {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
