use super::*;

impl Generator<ItemData> for StirringWardstone {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
