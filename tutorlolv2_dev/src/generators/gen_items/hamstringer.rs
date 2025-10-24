use super::*;

impl Generator<Item> for Hamstringer {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }