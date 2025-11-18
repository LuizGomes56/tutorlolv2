use super::*;

impl Generator<ItemData> for Hamstringer {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
