use super::*;

impl Generator<Item> for UmbralGlaive {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }