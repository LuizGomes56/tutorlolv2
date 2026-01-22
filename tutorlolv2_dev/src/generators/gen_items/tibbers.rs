use super::*;

impl Generator<ItemData> for TIBBERS {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
