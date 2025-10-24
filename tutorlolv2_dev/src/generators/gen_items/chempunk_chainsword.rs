use super::*;

impl Generator<Item> for ChempunkChainsword {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }