use super::*;

impl Generator<Item> for TheCollector {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }