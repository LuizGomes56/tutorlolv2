use super::*;

impl Generator<ItemData> for CorruptingPotion {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
