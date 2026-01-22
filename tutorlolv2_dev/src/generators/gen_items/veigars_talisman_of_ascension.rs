use super::*;

impl Generator<ItemData>
    for VeigarsTalismanOfAscension
{
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
