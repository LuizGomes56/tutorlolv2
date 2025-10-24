use super::*;

impl Generator<Item> for WardensEye {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }