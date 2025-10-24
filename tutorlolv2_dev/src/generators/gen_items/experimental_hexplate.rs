use super::*;

impl Generator<Item> for ExperimentalHexplate {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }