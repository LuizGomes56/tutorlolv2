use super::*;

impl Generator<Item> for HextechAlternator {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }