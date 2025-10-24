use super::*;

impl Generator<ItemData> for Lifeline {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
