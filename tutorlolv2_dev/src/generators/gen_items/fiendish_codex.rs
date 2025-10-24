use super::*;

impl Generator<Item> for FiendishCodex {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }