use super::*;

impl Generator<Item> for StatikkShiv {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }