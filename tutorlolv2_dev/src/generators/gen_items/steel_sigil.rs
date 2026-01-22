use super::*;

impl Generator<ItemData> for SteelSigil {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
