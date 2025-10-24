use super::*;

impl Generator<Item> for RodofAges {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }