use super::*;

impl Generator<ItemData> for LostChapter {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
