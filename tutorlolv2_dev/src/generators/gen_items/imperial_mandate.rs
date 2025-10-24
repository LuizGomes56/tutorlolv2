use super::*;

impl Generator<Item> for ImperialMandate {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }