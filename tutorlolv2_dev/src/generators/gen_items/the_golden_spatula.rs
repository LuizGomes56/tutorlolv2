use super::*;

impl Generator<Item> for TheGoldenSpatula {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }