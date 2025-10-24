use super::*;

impl Generator<Item> for PhantomDancer {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }