use super::*;

impl Generator<ItemData> for PawPrintPoisoner {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
