use super::*;

impl Generator<ItemData> for LegendaryMageItem {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
