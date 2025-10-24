use super::*;

impl Generator<Item> for StatBonus {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }