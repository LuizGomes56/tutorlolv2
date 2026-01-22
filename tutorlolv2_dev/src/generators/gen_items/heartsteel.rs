use super::*;

impl Generator<ItemData> for Heartsteel {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
