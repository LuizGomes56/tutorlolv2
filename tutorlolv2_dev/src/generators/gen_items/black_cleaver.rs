use super::*;

impl Generator<ItemData> for BlackCleaver {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
