use super::*;

impl Generator<ItemData>
    for BunnyPrimeBallista
{
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
