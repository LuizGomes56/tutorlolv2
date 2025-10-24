use super::*;

impl Generator<Item> for RanduinsOmen {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }