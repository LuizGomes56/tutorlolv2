use super::*;

impl Generator<ItemData> for LuckyDice {
    #[item_generator]
    fn generate(self: Box<Self>) -> MayFail<ItemData> {
        /* No implementation */
    }
}
