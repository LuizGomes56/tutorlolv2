use super::*;

impl Generator<Item> for Regicide {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }