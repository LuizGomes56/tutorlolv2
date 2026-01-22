use super::*;

impl Generator<ItemData> for Fulmination {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
