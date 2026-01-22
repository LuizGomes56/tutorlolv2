use super::*;

impl Generator<ItemData> for LastWhisper {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
