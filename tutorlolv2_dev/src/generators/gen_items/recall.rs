use super::*;

impl Generator<ItemData> for Recall {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
