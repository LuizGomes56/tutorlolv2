use super::*;

impl Generator<Item> for DarkSeal {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }