use super::*;

impl Generator<ItemData> for Galeforce {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
