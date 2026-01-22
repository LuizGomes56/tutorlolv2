use super::*;

impl Generator<ItemData>
    for MosstomperSeedling
{
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
