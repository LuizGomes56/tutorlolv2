use super::*;

impl Generator<ItemData> for Tiamat {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
