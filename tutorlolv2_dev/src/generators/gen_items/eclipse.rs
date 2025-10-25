use super::*;

impl Generator<ItemData> for Eclipse {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
