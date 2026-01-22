use super::*;

impl Generator<ItemData> for EnhancedLuckyDice {
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
        self.end()
    }
}
