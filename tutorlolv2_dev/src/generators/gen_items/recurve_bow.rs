use super::*;

impl Generator<Item> for RecurveBow {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }