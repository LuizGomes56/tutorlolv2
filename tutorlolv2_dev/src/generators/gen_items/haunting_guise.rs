use super::*;

impl Generator<Item> for HauntingGuise {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }