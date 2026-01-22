use super::*;

impl Generator<ItemData> for Bloodsong {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
