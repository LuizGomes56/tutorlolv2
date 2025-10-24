use super::*;

impl Generator<Item> for Sheen {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }