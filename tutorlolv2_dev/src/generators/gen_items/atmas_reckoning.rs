use super::*;

impl Generator<ItemData> for AtmasReckoning {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
