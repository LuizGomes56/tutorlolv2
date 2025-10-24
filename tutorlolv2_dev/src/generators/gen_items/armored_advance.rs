use super::*;

impl Generator<Item> for ArmoredAdvance {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }