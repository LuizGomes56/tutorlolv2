use super::*;

impl Generator<Item> for Thornmail {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }