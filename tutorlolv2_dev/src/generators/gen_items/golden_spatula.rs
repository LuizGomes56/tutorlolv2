use super::*;

impl Generator<ItemData> for GoldenSpatula {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
