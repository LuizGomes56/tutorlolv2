use super::*;

impl Generator<ItemData>
    for BandleJuiceOfPower
{
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
