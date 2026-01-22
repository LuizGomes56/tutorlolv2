use super::*;

impl Generator<ItemData> for BraveryVoucher {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
