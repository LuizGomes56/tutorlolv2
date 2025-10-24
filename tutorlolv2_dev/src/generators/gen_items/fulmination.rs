use super::*;

impl Generator<ItemData> for Fulmination {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
