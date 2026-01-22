use super::*;

impl Generator<ItemData> for Stormrazor {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
