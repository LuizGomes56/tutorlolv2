use super::*;

impl Generator<ItemData>
    for PrismaticStatVoucher
{
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
