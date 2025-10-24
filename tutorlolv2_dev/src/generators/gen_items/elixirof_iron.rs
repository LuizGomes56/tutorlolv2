use super::*;

impl Generator<Item> for ElixirofIron {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }