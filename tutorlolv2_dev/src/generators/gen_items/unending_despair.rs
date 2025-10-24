use super::*;

impl Generator<Item> for UnendingDespair {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }