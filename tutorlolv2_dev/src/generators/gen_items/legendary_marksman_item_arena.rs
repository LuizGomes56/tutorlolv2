use super::*;

impl Generator<ItemData> for LegendaryMarksmanItemArena {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
