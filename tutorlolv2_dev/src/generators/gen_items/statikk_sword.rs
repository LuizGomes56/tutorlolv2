use super::*;

impl Generator<ItemData> for StatikkSword {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
