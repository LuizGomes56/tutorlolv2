use super::*;

impl Generator<Item> for Lifeline {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }