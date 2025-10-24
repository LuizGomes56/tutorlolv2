use super::*;

impl Generator<Item> for Tiamat {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }