use super::*;

impl Generator<ItemData> for HollowRadiance {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
