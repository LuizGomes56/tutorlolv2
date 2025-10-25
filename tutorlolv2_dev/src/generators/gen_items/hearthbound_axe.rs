use super::*;

impl Generator<ItemData> for HearthboundAxe {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
