use super::*;

impl Generator<ItemData> for ObsidianEdge {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
