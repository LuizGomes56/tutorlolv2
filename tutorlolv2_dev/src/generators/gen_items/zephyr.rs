use super::*;

impl Generator<ItemData> for Zephyr {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
