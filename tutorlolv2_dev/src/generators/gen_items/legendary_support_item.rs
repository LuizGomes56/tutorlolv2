use super::*;

impl Generator<ItemData> for LegendarySupportItem {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
