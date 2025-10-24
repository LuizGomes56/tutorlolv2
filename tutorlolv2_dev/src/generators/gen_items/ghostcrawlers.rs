use super::*;

impl Generator<Item> for Ghostcrawlers {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }