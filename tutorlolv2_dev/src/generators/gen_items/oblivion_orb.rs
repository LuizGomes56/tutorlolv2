use super::*;

impl Generator<ItemData> for OblivionOrb {
    fn generate(
        self: Box<Self>,
    ) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
