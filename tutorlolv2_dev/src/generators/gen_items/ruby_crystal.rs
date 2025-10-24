use super::*;

impl Generator<Item> for RubyCrystal {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }