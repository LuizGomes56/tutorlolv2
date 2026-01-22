use super::*;

impl Generator<ItemData>
    for FarsightAlteration
{
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
