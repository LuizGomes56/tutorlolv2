use super::*;

impl Generator<ItemData> for DemonicEmbrace {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
