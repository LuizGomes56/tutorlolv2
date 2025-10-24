use super::*;

impl Generator<Item> for HextechRocketbelt {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }