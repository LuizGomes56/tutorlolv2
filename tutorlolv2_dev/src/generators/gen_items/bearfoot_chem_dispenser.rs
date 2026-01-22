use super::*;

impl Generator<ItemData>
    for BearfootChemDispenser
{
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
