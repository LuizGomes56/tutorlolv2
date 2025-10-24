use super::*;

impl Generator<Item> for HearthboundAxe {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }