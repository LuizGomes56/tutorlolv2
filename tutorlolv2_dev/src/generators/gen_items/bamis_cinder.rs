use super::*;

impl Generator<ItemData> for BamisCinder {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
