use super::*;

impl Generator<Item> for HealthPotion {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }