use super::*;

impl Generator<ItemData> for HarrowingCrescent {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
