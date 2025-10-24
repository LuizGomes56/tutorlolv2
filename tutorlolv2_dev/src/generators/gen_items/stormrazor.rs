use super::*;

impl Generator<Item> for Stormrazor {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }