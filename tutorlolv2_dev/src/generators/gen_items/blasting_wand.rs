use super::*;

impl Generator<Item> for BlastingWand {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }