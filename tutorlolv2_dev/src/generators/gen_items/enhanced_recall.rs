use super::*;

impl Generator<ItemData> for EnhancedRecall {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
