use super::*;

impl Generator<Item> for LegendarySupportItem {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }