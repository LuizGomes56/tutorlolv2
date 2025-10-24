use super::*;

impl Generator<Item> for EnhancedLuckyDice {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }