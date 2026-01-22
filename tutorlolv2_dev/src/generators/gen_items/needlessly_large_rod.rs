use super::*;

impl Generator<ItemData>
    for NeedlesslyLargeRod
{
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
