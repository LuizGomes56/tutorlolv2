use super::*;

impl Generator<ItemData> for Shadowflame {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
