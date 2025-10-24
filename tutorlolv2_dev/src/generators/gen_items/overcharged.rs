use super::*;

impl Generator<Item> for Overcharged {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }