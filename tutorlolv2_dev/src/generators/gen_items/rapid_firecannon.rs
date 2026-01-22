use super::*;

impl Generator<ItemData> for RapidFirecannon {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
