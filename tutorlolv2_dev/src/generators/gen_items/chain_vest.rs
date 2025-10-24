use super::*;

impl Generator<ItemData> for ChainVest {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
