use super::*;

impl Generator<Item> for VoidStaff {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }