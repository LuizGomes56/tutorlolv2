use super::*;

impl Generator<ItemData> for LifewellPendant {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
