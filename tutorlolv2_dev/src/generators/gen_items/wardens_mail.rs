use super::*;

impl Generator<ItemData> for WardensMail {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
