use super::*;

impl Generator<ItemData> for RodOfAges {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
