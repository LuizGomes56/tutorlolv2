use super::*;

impl Generator<Item> for ScarecrowEffigy {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }