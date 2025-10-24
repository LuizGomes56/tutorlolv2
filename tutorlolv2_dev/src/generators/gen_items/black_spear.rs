use super::*;

impl Generator<Item> for BlackSpear {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }