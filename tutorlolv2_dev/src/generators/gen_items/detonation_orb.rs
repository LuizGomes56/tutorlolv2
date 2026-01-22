use super::*;

impl Generator<ItemData> for DetonationOrb {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
