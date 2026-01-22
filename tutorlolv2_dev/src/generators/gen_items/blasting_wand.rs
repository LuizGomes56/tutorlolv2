use super::*;

impl Generator<ItemData> for BlastingWand {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
