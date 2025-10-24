use super::*;

impl Generator<Item> for Noonquiver {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }