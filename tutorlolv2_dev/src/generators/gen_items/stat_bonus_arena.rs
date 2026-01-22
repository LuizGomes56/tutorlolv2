use super::*;

impl Generator<ItemData> for StatBonusArena {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
