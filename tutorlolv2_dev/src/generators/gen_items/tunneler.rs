use super::*;

impl Generator<ItemData> for Tunneler {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
