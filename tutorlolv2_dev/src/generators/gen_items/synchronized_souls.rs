use super::*;

impl Generator<ItemData> for SynchronizedSouls {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
