use super::*;

impl Generator<ItemData> for HoppedUpHex {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
