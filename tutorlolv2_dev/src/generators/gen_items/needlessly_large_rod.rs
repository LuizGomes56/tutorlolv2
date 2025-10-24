use super::*;

impl Generator<ItemData> for NeedlesslyLargeRod {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
