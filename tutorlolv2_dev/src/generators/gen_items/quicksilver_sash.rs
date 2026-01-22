use super::*;

impl Generator<ItemData> for QuicksilverSash {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
