use super::*;

impl Generator<Item> for DreamMaker {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }