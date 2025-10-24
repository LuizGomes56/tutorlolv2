use super::*;

impl Generator<Item> for Morellonomicon {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }