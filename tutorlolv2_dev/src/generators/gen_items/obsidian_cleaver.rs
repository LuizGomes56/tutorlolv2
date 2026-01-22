use super::*;

impl Generator<ItemData> for ObsidianCleaver {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
