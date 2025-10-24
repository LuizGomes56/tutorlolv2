use super::*;

impl Generator<Item> for WardensMail {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }