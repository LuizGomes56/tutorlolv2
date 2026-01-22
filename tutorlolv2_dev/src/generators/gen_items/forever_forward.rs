use super::*;

impl Generator<ItemData> for ForeverForward {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
