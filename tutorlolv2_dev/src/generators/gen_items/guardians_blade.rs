use super::*;

impl Generator<Item> for GuardiansBlade {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }