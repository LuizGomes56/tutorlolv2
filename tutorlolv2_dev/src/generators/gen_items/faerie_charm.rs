use super::*;

impl Generator<Item> for FaerieCharm {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }