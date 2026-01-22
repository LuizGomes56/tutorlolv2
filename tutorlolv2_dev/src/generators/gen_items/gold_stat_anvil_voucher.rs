use super::*;

impl Generator<ItemData>
    for GoldStatAnvilVoucher
{
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
