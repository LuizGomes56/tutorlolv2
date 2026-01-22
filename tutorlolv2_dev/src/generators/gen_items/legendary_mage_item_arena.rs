use super::*;

impl Generator<ItemData> for LegendaryMageItemArena {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
