use super::*;

impl Generator<Item> for PlatedSteelcaps {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }