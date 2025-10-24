use super::*;

impl Generator<ItemData> for PhantomDancer {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
