use super::*;

impl Generator<ItemData> for GlacialBuckler {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
