use super::*;

impl Generator<ItemData>
    for ChainlacedCrushers
{
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
