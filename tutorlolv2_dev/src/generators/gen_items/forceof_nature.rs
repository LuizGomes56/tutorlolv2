use super::*;

impl Generator<Item> for ForceofNature {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }