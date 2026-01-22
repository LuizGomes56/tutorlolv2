use super::*;

impl Generator<ItemData>
    for ShurelyasBattlesong
{
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
