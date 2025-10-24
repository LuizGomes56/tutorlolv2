use super::*;

impl Generator<Item> for Dragonheart {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }