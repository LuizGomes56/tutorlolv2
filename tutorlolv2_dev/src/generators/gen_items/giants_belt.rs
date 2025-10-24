use super::*;

impl Generator<Item> for GiantsBelt {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }