use super::*;

impl Generator<Item> for Shadowflame {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }