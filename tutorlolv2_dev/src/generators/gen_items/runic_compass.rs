use super::*;

impl Generator<Item> for RunicCompass {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }