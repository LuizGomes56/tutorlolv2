use super::*;

impl Generator<Item> for LegendaryMarksmanItem {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }