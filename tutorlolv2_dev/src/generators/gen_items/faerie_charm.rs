use super::*;

impl Generator<ItemData> for FaerieCharm {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
