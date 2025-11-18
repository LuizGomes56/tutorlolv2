use super::*;

impl Generator<ItemData> for Lifeline {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
