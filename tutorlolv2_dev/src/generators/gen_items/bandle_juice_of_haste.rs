use super::*;

impl Generator<ItemData>
    for BandleJuiceOfHaste
{
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
