use super::*;

impl Generator<ItemData> for SorcerersShoes {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
