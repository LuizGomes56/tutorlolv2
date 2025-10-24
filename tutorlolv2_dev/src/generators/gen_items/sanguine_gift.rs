use super::*;

impl Generator<Item> for SanguineGift {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }