use super::*;

impl Generator<ItemData> for RedemptionU32 {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
