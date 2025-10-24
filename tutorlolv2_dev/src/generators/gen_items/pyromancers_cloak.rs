use super::*;

impl Generator<Item> for PyromancersCloak {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }