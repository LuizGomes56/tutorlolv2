use super::*;

impl Generator<Item> for LegendaryFighterItem {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }