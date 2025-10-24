use super::*;

impl Generator<Item> for ChainVest {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }