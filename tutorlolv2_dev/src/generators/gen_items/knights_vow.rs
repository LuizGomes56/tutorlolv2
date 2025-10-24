use super::*;

impl Generator<Item> for KnightsVow {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }