use super::*;

impl Generator<ItemData> for GiantsBelt {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
