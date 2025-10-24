use super::*;

impl Generator<Item> for Opportunity {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }