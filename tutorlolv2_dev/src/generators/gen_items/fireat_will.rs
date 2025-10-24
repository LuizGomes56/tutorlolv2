use super::*;

impl Generator<Item> for FireatWill {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }