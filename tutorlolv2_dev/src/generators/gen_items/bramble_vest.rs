use super::*;

impl Generator<Item> for BrambleVest {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }