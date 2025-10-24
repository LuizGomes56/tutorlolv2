use super::*;

impl Generator<Item> for ArchangelsStaff {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }