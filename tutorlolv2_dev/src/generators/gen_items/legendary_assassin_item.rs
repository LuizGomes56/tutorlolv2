use super::*;

impl Generator<Item> for LegendaryAssassinItem {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }