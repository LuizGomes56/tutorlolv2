use super::*;

impl Generator<ItemData> for DoransBlade {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
