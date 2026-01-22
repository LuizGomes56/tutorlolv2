use super::*;

impl Generator<ItemData> for AetherWisp {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
