use super::*;

impl Generator<Item> for Gusto {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }