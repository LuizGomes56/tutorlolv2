use super::*;

impl Generator<Item> for Phage {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }