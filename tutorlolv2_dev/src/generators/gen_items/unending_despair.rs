use super::*;

impl Generator<ItemData> for UnendingDespair {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
