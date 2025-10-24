use super::*;

impl Generator<Item> for WitsEnd {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }