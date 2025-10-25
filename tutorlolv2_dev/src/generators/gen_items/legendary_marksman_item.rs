use super::*;

impl Generator<ItemData> for LegendaryMarksmanItem {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
