use super::*;

impl Generator<ItemData> for BlightingJewel {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
