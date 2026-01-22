use super::*;

impl Generator<ItemData>
    for DiamondTippedSpear
{
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
