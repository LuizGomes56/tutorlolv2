use super::*;

impl Generator<ItemData>
    for SwordOfBlossomingDawn
{
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
