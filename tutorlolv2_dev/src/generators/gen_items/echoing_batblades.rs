use super::*;

impl Generator<ItemData> for EchoingBatblades {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
