use super::*;

impl Generator<Item> for LongSword {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }