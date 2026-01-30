use super::*;

impl Generator<ItemData> for GuidingHex {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
