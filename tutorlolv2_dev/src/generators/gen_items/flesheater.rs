use super::*;

impl Generator<Item> for Flesheater {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }