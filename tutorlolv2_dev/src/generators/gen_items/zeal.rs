use super::*;

impl Generator<ItemData> for Zeal {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
