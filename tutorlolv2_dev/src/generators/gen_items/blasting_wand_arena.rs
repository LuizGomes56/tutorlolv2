use super::*;

impl Generator<ItemData> for BlastingWandArena {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
