use super::*;

impl Generator<Item> for RefillablePotion {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }