use super::*;

impl Generator<Item> for Dagger {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }