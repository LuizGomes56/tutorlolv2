use super::*;

impl Generator<ItemData> for LeechingLeer {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
