use super::*;

impl Generator<ItemData> for VortexGlove {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
