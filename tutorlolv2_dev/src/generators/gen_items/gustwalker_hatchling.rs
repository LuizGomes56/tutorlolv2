use super::*;

impl Generator<ItemData>
    for GustwalkerHatchling
{
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
