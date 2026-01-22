use super::*;

impl Generator<ItemData> for SavageSlice {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
