use super::*;

impl Generator<Item> for ForceofEntropy {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }