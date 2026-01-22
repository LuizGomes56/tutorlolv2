use super::*;

impl Generator<ItemData> for ReapersToll {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
