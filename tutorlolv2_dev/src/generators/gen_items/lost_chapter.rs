use super::*;

impl Generator<Item> for LostChapter {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }