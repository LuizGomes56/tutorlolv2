use super::*;

impl Generator<ItemData>
    for TalismanOfAscension
{
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
