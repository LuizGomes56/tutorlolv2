use super::*;

impl Generator<Item> for Trailblazer {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }