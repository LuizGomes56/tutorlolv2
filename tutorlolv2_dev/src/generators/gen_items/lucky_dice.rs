use super::*;

impl Generator<ItemData> for LuckyDice {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
