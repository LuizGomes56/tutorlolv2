use super::*;

impl Generator<Item> for DiamondTippedSpear {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }