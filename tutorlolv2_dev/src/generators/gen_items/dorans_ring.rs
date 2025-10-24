use super::*;

impl Generator<Item> for DoransRing {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }