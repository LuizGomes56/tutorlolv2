use super::*;

impl Generator<Item> for Hullbreaker {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }