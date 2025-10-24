use super::*;

impl Generator<Item> for SpiritVisage {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }