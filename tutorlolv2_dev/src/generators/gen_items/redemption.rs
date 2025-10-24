use super::*;

impl Generator<Item> for Redemption {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }