use super::*;

impl Generator<Item> for DeathsDaughter {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }