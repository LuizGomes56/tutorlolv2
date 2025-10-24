use super::*;

impl Generator<Item> for Swiftmarch {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }