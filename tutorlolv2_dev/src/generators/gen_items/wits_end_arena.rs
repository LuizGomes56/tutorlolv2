use super::*;

impl Generator<ItemData> for WitsEndArena {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
