use super::*;

impl Generator<ItemData> for NullMagicMantle {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
