use super::*;

impl Generator<ItemData> for TheCollector {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
