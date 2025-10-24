use super::*;

impl Generator<Item> for Decapitator {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }