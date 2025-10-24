use super::*;

impl Generator<Item> for Cruelty {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }