use super::*;

impl Generator<ItemData> for FaerieCharm {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
