use super::*;

impl Generator<ItemData> for Riftmaker {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
