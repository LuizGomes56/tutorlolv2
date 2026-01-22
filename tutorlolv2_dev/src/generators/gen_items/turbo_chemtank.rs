use super::*;

impl Generator<ItemData> for TurboChemtank {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
