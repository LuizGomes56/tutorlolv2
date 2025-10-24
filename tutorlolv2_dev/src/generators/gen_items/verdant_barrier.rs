use super::*;

impl Generator<Item> for VerdantBarrier {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }