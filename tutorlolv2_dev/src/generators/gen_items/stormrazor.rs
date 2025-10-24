use super::*;

impl Generator<ItemData> for Stormrazor {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
