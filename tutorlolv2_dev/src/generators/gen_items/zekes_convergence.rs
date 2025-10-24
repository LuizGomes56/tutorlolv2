use super::*;

impl Generator<Item> for ZekesConvergence {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }