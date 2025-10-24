use super::*;

impl Generator<Item> for WordlessPromise {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }