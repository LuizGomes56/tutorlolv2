use super::*;

impl Generator<ItemData> for BlightingJewel {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
