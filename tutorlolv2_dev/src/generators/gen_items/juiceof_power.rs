use super::*;

impl Generator<Item> for JuiceofPower {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }