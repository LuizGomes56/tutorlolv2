use super::*;

impl Generator<ItemData>
    for PrumbissElectrocarver
{
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
