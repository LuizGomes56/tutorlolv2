use super::*;

impl Generator<Item> for YourCut {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }