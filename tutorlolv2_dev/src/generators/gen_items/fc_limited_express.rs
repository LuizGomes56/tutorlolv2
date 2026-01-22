use super::*;

impl Generator<ItemData> for FcLimitedExpress {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
