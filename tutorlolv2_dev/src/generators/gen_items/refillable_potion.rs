use super::*;

impl Generator<ItemData> for RefillablePotion {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
