use super::*;

impl Generator<ItemData> for NeverendingMobstomper {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
