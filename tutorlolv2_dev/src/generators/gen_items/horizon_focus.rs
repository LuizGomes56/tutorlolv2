use super::*;

impl Generator<Item> for HorizonFocus {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }