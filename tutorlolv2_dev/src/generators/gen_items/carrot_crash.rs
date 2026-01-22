use super::*;

impl Generator<ItemData> for CarrotCrash {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
