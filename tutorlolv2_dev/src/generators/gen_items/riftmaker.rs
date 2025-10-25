use super::*;

impl Generator<ItemData> for Riftmaker {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
