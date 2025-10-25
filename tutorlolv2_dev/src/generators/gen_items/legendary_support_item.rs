use super::*;

impl Generator<ItemData> for LegendarySupportItem {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
