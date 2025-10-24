use super::*;

impl Generator<Item> for ForeverForward {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }