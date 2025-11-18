use super::*;

impl Generator<ItemData> for Hexdrinker {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
