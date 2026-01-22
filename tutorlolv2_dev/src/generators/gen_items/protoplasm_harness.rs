use super::*;

impl Generator<ItemData> for ProtoplasmHarness {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
