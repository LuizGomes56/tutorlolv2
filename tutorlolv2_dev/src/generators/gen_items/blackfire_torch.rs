use super::*;

impl Generator<Item> for BlackfireTorch {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }