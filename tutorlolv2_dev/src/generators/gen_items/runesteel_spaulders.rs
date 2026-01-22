use super::*;

impl Generator<ItemData>
    for RunesteelSpaulders
{
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
