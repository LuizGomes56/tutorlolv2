use super::*;

impl Generator<Item> for TheBrutalizer {
            #[item_generator]
            fn generate(mut self: Box<Self>) -> MayFail<Item> {}
        }