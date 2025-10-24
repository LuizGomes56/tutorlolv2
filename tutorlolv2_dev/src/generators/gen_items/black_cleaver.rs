use super::*;

impl Generator<ItemData> for BlackCleaver {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
