use super::*;

impl Generator<Item> for Runecarver {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }