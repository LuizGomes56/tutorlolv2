use super::*;

impl Generator<Item> for Dawncore {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }