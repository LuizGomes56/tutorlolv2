use super::*;

impl Generator<Item> for DoransShield {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }