use super::*;

impl Generator<ItemData> for KrakenSlayer {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
