use super::*;

impl Generator<ItemData> for Yuumibot {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
