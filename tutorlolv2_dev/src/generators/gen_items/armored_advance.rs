use super::*;

impl Generator<ItemData> for ArmoredAdvance {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
