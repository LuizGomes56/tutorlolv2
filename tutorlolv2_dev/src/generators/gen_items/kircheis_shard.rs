use super::*;

impl Generator<ItemData> for KircheisShard {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
