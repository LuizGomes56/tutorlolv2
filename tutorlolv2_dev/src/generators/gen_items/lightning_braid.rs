use super::*;

impl Generator<ItemData> for LightningBraid {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
