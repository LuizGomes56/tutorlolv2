use super::*;

impl Generator<Item> for Zeal {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }