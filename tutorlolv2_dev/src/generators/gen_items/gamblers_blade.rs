use super::*;

impl Generator<Item> for GamblersBlade {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }