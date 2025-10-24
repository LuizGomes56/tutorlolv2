use super::*;

impl Generator<Item> for Kindlegem {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }