use super::*;

impl Generator<ItemData> for WatchfulWardstone {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
