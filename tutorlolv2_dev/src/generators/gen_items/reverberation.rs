use super::*;

impl Generator<Item> for Reverberation {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }