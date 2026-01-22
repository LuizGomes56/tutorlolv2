use super::*;

impl Generator<ItemData> for Cryptbloom {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
