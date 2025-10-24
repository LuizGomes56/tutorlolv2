use super::*;

impl Generator<ItemData> for Bloodthirster {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
