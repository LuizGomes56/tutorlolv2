use super::*;

impl Generator<Item> for Everfrost {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }