use super::*;

impl Generator<ItemData> for LightningRod {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
