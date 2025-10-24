use super::*;

impl Generator<Item> for Zephyr {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }