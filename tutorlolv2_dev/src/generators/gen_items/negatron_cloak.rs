use super::*;

impl Generator<ItemData> for NegatronCloak {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
