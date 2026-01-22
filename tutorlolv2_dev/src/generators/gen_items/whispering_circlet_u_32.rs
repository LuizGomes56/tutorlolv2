use super::*;

impl Generator<ItemData> for WhisperingCircletU32 {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
