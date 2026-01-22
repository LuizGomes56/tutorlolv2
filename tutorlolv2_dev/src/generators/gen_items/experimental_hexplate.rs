use super::*;

impl Generator<ItemData>
    for ExperimentalHexplate
{
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
