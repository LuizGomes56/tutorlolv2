use super::*;

impl Generator<Item> for DeadMansPlate {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }