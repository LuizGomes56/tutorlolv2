use super::*;

impl Generator<Item> for Eclipse {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }