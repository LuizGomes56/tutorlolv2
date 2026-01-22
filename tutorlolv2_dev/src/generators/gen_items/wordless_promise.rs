use super::*;

impl Generator<ItemData> for WordlessPromise {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
