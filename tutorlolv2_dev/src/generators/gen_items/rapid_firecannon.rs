use super::*;

impl Generator<Item> for RapidFirecannon {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }