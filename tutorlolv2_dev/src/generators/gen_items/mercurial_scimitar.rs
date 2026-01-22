use super::*;

impl Generator<ItemData> for MercurialScimitar {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
