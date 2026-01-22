use super::*;

impl Generator<ItemData>
    for ChempunkChainsword
{
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
