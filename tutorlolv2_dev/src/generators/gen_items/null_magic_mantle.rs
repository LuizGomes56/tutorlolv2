use super::*;

impl Generator<Item> for NullMagicMantle {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }