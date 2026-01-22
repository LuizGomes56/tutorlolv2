use super::*;

impl Generator<ItemData> for PhantomDancer {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
