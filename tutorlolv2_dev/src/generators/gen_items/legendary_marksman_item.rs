use super::*;

impl Generator<ItemData> for LegendaryMarksmanItem {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
