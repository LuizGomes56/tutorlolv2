use super::*;

impl Generator<ItemData> for EndlessHunger {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
