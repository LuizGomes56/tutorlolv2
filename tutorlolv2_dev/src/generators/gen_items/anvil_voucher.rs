use super::*;

impl Generator<ItemData> for AnvilVoucher {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
