use super::*;

impl Generator<ItemData> for TargonsBuckler {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
