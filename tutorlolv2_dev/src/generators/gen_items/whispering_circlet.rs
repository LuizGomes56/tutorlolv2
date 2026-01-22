use super::*;

impl Generator<ItemData> for WhisperingCirclet {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
