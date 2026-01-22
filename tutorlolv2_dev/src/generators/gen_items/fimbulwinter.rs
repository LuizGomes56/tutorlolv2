use super::*;

impl Generator<ItemData> for Fimbulwinter {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
