use super::*;

impl Generator<Item> for Tunneler {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }