use super::*;

impl Generator<ItemData> for LegendaryMageItem {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
