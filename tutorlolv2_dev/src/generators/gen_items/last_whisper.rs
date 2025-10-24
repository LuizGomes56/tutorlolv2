use super::*;

impl Generator<Item> for LastWhisper {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }