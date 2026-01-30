use super::*;

impl Generator<ItemData> for GoldStatAnvilVoucherArena {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
