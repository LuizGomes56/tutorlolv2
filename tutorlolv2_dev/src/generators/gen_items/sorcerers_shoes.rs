use super::*;

impl Generator<Item> for SorcerersShoes {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }