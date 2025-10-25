use super::*;

impl Generator<ItemData> for WordlessPromise {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
