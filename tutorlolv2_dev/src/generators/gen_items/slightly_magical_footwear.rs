use super::*;

impl Generator<ItemData>
    for SlightlyMagicalFootwear
{
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
