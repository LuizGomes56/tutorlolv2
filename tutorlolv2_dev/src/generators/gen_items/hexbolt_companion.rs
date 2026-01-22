use super::*;

impl Generator<ItemData> for HexboltCompanion {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
