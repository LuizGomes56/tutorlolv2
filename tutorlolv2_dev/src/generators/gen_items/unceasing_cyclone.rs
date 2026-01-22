use super::*;

impl Generator<ItemData> for UnceasingCyclone {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
