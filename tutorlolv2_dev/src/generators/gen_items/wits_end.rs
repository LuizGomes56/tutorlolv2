use super::*;

impl Generator<ItemData> for WitsEnd {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
