use super::*;

impl Generator<Item> for LegendaryMageItem {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }