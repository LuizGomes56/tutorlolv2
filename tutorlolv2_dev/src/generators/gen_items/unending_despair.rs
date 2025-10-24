use super::*;

impl Generator<ItemData> for UnendingDespair {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
