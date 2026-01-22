use super::*;

impl Generator<ItemData> for CosmicDrive {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
