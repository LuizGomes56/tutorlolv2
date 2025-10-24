use super::*;

impl Generator<Item> for GargoyleStoneplate {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }