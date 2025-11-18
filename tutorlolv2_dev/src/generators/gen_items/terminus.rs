use super::*;

impl Generator<ItemData> for Terminus {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
