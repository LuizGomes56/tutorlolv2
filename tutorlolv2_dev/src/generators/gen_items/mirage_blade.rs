use super::*;

impl Generator<Item> for MirageBlade {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }