use super::*;

impl Generator<ItemData> for Overgrowth {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
