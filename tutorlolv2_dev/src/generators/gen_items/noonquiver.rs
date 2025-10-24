use super::*;

impl Generator<ItemData> for Noonquiver {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
