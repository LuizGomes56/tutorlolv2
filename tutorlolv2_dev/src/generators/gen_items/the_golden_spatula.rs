use super::*;

impl Generator<ItemData> for TheGoldenSpatula {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
