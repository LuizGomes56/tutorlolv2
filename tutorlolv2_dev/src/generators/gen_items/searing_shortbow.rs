use super::*;

impl Generator<ItemData> for SearingShortbow {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
