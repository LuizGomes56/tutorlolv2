use super::*;

impl Generator<Item> for ControlWard {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }