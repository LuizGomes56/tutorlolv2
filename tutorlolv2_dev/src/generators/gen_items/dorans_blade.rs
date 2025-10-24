use super::*;

impl Generator<Item> for DoransBlade {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }