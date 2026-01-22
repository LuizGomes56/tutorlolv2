use super::*;

impl Generator<ItemData> for Kindlegem {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
