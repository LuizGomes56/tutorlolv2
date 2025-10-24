use super::*;

impl Generator<Item> for SapphireCrystal {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }