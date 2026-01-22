use super::*;

impl Generator<ItemData> for DemonicEmbrace {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
