use super::*;

impl Generator<ItemData> for ForeverForward {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
