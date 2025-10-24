use super::*;

impl Generator<Item> for Goredrinker {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }