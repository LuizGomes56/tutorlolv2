use super::*;

impl Generator<ItemData> for DreamMaker {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
