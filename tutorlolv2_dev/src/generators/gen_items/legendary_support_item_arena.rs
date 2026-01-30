use super::*;

impl Generator<ItemData> for LegendarySupportItemArena {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
