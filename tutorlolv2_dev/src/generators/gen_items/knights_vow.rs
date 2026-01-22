use super::*;

impl Generator<ItemData> for KnightsVow {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
