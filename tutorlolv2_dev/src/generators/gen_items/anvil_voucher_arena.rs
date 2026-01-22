use super::*;

impl Generator<ItemData> for AnvilVoucherArena {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
