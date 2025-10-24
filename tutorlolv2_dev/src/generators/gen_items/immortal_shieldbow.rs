use super::*;

impl Generator<Item> for ImmortalShieldbow {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }