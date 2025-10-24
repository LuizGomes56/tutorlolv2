use super::*;

impl Generator<Item> for Rectrix {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }